import { Universe } from "spreed-it";
import Chartist from "chartist";

import "chartist/dist/chartist.min.css";
import "./app.css";

const width = 800;
const height = 600;
const universe = Universe.new(width, height, 100);

const canvas = document.getElementById("spreed-it-canvas");
canvas.height = height;
canvas.width = width;

const susceptibleCount = document.querySelector("#susceptible-count");
const infectedCount = document.querySelector("#infected-count");
const removedCount = document.querySelector("#removed-count");
const diedCount = document.querySelector("#died-count");

var data = {
    // A labels array that can contain any sort of values
    labels: ['Mon', 'Tue', 'Wed', 'Thu', 'Fri'],
    // Our series array that contains series objects or in this case series data arrays
    series: [
      [5, 2, 4, 2, 0]
    ]
  };
  
  // Create a new line chart object where as first parameter we pass in a selector
  // that is resolving to our chart container element. The Second parameter
  // is the actual data object.
  new Chartist.Line("#chart", data);

const ctx = canvas.getContext("2d");

const renderLoop = () => {
    universe.tick();
    universe.render(ctx);

    susceptibleCount.textContent = universe.susceptible();
    infectedCount.textContent = universe.infected();
    removedCount.textContent = universe.removed();
    diedCount.textContent = universe.died();

    requestAnimationFrame(renderLoop);
}

requestAnimationFrame(renderLoop);