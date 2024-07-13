import init, { trick_png } from './rust_eyesart.js';

async function run() {
    await init();
    document.getElementById("mainImgL").src = trick_png(0);
    document.getElementById("mainImgR").src = trick_png(0);

    document.getElementById("left_arrow").addEventListener("click",()=>{
        document.getElementById("mainImgL").src = trick_png(1);
        document.getElementById("mainImgR").src = trick_png(1);
    });
    document.getElementById("down_arrow").addEventListener("click",()=>{
        document.getElementById("mainImgL").src = trick_png(2);
        document.getElementById("mainImgR").src = trick_png(2);
    });
    document.getElementById("up_arrow").addEventListener("click",()=>{
        document.getElementById("mainImgL").src = trick_png(3);
        document.getElementById("mainImgR").src = trick_png(3);
    });
    document.getElementById("right_arrow").addEventListener("click",()=>{
        document.getElementById("mainImgL").src = trick_png(4);
        document.getElementById("mainImgR").src = trick_png(4);
    });
}
run();

