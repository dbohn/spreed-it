import { Universe } from "spreed-it";
import Chartist from "chartist";

import "chartist/dist/chartist.min.css";
import "./app.css";

const width = 800;
const height = 600;
const universe = Universe.new(width, height, 200);

const canvas = document.getElementById("spreed-it-canvas");
canvas.height = height;
canvas.width = width;

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

let chartUpdateInterval = window.setInterval(() => {
    chart.update({ series: getSeries() });
}, 100);

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

        this.susceptibleCount = document.querySelector("#susceptible-count");
        this.infectedCount = document.querySelector("#infected-count");
        this.removedCount = document.querySelector("#removed-count");
        this.diedCount = document.querySelector("#died-count");

        document.querySelector("#trigger-button").addEventListener('click', (e) => {
            this.run();
            document.querySelector("#overlay").style.display = "none";
        });
    }
    getContext() {
        if (this.ctx === null) {
            this.ctx = this.canvas.getContext("2d");
        }

        return this.ctx;
    }

    run() {
        this.universe = Universe.new(this.width, this.height, 100);

        if (!this.rendering) {
            this.rendering = true;
            requestAnimationFrame(() => this.render());
        }
    }

    render() {
        this.universe.tick();
        this.universe.render(this.getContext());

        healthStatus.forEach(status => {
            const val = this.universe[status]();
            if (status === "infected" && val === 0 && chartUpdateInterval) {
                window.clearInterval(chartUpdateInterval);
                chartUpdateInterval = null;
                setTimeout(() => {
                    document.querySelector("#restart-overlay").classList.remove("hidden");
                }, 1000);
            }
            const series = stats[status].series;
            stats[status].el.textContent = val;
            series.push(val);
        });

        requestAnimationFrame(() => this.render());
    }
}


const app = new App(document.getElementById("spreed-it-canvas"), 800, 600);
app.init();