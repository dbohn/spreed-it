import { Universe } from "spreed-it";

const width = 64 * 6 + 1;
const height = 64 * 6 + 1;
const universe = Universe.new(width, height);

const canvas = document.getElementById("spreed-it-canvas");
canvas.height = height;
canvas.width = width;

const ctx = canvas.getContext('2d');

const renderLoop = () => {
    universe.tick();
    universe.render(ctx);

    requestAnimationFrame(renderLoop);
}

requestAnimationFrame(renderLoop);