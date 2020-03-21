import { Universe } from "spreed-it";

const width = 800;
const height = 600;
const universe = Universe.new(width, height, 20);

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