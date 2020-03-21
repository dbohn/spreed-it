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
    high: 100,
    showPoint: false,
    showArea: true,
    showLine: false,
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

const ctx = canvas.getContext("2d");

const renderLoop = () => {
    universe.tick();
    universe.render(ctx);

    healthStatus.forEach(status => {
        const val = universe[status]();
        if (status === "infected" && val === 0 && chartUpdateInterval) {
            window.clearInterval(chartUpdateInterval);
            chartUpdateInterval = null;
        }
        const series = stats[status].series;
        stats[status].el.textContent = val;
        series.push(val);
    });

    requestAnimationFrame(renderLoop);
}

requestAnimationFrame(renderLoop);