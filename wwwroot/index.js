import init, { pixel_filter } from './demo_wasm_pub.js';

const video = document.getElementById('myVideo');
const canvas = document.getElementById('myCanvas');
const ctx = canvas.getContext('2d',{willReadFrequently: true,});

navigator.mediaDevices.getUserMedia({ video: true, audio: false })
    .then(stream => {
        video.srcObject = stream;
        // 描画を開始
        video.addEventListener('loadeddata', () => {            
            canvas.width = video.videoWidth;
            canvas.height = video.videoHeight;
            function draw() {
                ctx.drawImage(video, 0, 0, canvas.width, canvas.height);
                requestAnimationFrame(draw);
            }
            draw();
        });
    })
    .catch(err => {
        console.error('エラー:', err);
    });
// window.addEventListener("load",(event)=>{
//     for(let i=0;i< 54;i++){
//         for(let j=0;j< 40;j++){
//             const chk = document.createElement("input");
//             chk.type = "checkbox";
//             chk.dataset.row=i;
//             chk.dataset.column=j;
//
//         }
//     }
// });
async function run() {
    await init();
    document.getElementById("pixel_filter").addEventListener("click", () => {
        canvas.width = video.videoWidth;
        canvas.height = video.videoHeight;
        function draw() {
            const imageData = ctx.getImageData(0, 0, canvas.width, canvas.height);
            const ret = pixel_filter(new Uint8Array(imageData.data.buffer),canvas.width,canvas.height);
            // for canvas
            ctx.putImageData(new ImageData(new Uint8ClampedArray(ret.bytes.buffer), canvas.width, canvas.height), 0, 0);
            // for chk
            setInterval(()=>{console.log(ret.text);},2000)
            // document.getElementById("container").innerText= ret.text;
            requestAnimationFrame(draw);
        }
        draw();
    });
}
run();
