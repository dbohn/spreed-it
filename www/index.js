import { Universe } from "spreed-it";
import Chartist from "chartist";

import "chartist/dist/chartist.min.css";
import "./app.css";

const healthStatus = ["susceptible", "infected", "removed", "died"];
const stats = healthStatus.reduce((base, key) => {
    base[key] = {
        el: document.querySelector(`#${key}-count`),
        series: []
    };
    return base;
}, {});

const chart = new Chartist.Line("#chart", {
    series: [[], [], []]
}, {
    showPoint: false,
    showArea: true,
    showLine: false,
    chartPadding: {
        top: 0,
        right: 0,
        left: 0,
        bottom: 0
    },
    axisX: {
        showGrid: false,
        showLabel: false
    },
    axisY: {
        showGrid: false,
        showLabel: false
    }
});

const getSeries = () => {
    const offset = [];
    return healthStatus.slice().reverse().map(status => {
        return stats[status].series.map((v, idx) => {
            const stackedValue = v + (offset[idx] || 0);
            offset[idx] = stackedValue;
            return stackedValue;
        });
    }).reverse();
};

class App {
    constructor(canvas, width, height) {
        this.canvas = canvas;
        this.width = width;
        this.height = height;
        this.universe = null;
        this.ctx = null;
        this.rendering = false;
    }

    init() {
        this.canvas.width = this.width;
        this.canvas.height = this.height;

        document.querySelectorAll("#trigger-button, #overlay").forEach((item) => {
            item.addEventListener('click', () => {
                this.run();
            });
        });
    }

    getContext() {
        if (this.ctx === null) {
            this.ctx = this.canvas.getContext("2d");
        }

        return this.ctx;
    }

    run() {
        document.querySelector("#overlay").classList.add("hidden");
        this.universe = Universe.new(this.width, this.height, 100);

        this.initChart();

        if (!this.rendering) {
            this.rendering = true;
            requestAnimationFrame(() => this.render());
        }
    }

    initChart() {
        for (const key in stats) {
            if (stats.hasOwnProperty(key)) {
                const element = stats[key];
                element.series = [];
            }
        }

        this.chartUpdateInterval = window.setInterval(() => {
            chart.update({ series: getSeries() });
        }, 100);
    }

    render() {
        this.universe.tick();
        this.universe.render(this.getContext());

        healthStatus.forEach(status => {
            const val = this.universe[status]();
            const series = stats[status].series;
            stats[status].el.textContent = val;
            series.push(val);

            if (status === "infected" && val === 0 && this.chartUpdateInterval) {
                setTimeout(() => {
                    window.clearInterval(this.chartUpdateInterval);
                    this.chartUpdateInterval = null;
                    document.querySelector("#overlay").classList.remove("hidden");
                }, 1000);
            }
        });

        requestAnimationFrame(() => this.render());
    }
}


const app = new App(document.getElementById("spreed-it-canvas"), 800, 600);
app.init();