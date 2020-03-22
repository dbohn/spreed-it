import { Universe, AgeGroup } from "spreed-it";
import Chartist from "chartist";
import Vue from "vue";

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
    constructor(width, height) {
        this.width = width;
        this.height = height;
        this.universe = null;
        this.ctx = null;
        this.rendering = false;
    }

    init(canvas) {
        this.canvas = canvas;
        this.canvas.width = this.width;
        this.canvas.height = this.height;
    }

    getContext() {
        if (this.ctx === null) {
            this.ctx = this.canvas.getContext("2d");
        }

        return this.ctx;
    }

    run(ageGroups) {
        document.querySelector("#overlay").classList.add("hidden");
        this.universe = Universe.new(this.width, this.height, 0);

        ageGroups.forEach((ageGroup) => {
            this.universe.spawn_age_group(ageGroup);
        });

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


const app = new App(800, 600);

const vue = new Vue({
    el: '#app',
    data: {
        ageGroups: [
            {
                label: '< 18 Jahre',
                config: {
                    size: 10,
                    activity: 2,
                    vulnerability: 0.6,
                    letality: 0.01,
                }
            },
            {
                label: '18 - 24 Jahre',
                config: {
                    size: 10,
                    activity: 1.9,
                    vulnerability: 0.7,
                    letality: 0.01,
                }
            },
            {
                label: '25 - 40 Jahre',
                config: {
                    size: 10,
                    activity: 1.1,
                    vulnerability: 0.8,
                    letality: 0.02,
                }
            },
            {
                label: '41 - 64 Jahre',
                config: {
                    size: 30,
                    activity: 1.0,
                    vulnerability: 0.9,
                    letality: 0.05,
                }
            },
            {
                label: '≥ 65 Jahre',
                config: {
                    size: 10,
                    activity: 0.7,
                    vulnerability: 1.0,
                    letality: 0.1,
                }
            }
        ]
    },

    mounted() {
        app.init(this.$refs.canvas);
    },

    methods: {
        run() {
            app.run(
                this.ageGroups.map(({config}) => AgeGroup.new(config.size, config.activity, config.vulnerability, config.letality))
            );
        }
    }
});