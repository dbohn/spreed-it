import { Universe } from "spreed-it";
import Chartist from "chartist";

import "chartist/dist/chartist.min.css";
import "./app.css";

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

        this.susceptibleCount.textContent = this.universe.susceptible();
        this.infectedCount.textContent = this.universe.infected();
        this.removedCount.textContent = this.universe.removed();
        this.diedCount.textContent = this.universe.died();

        requestAnimationFrame(() => this.render());
    }
}


const app = new App(document.getElementById("spreed-it-canvas"), 800, 600);
app.init();