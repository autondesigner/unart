const fs = require('fs');
let version = 'a0';
if (process.argv.length > 2) {
  version = process.argv[2];
}
let imageIndex = 0;
if (process.argv.length > 3) {
  imageIndex = process.argv[3];
}
const imgDir = "./render/" + version;

const resizedDir = `./render/${version}-resized`;

const imageWidth = 2048;

if (!fs.existsSync(resizedDir)) {
  fs.mkdirSync(resizedDir);
}

const child_process = require('child_process');
//ffmpeg -i input.jpg -vf scale=320:240 output_320x240.png

child_process.spawnSync(
  'ffmpeg',
  [
    '-i',
    `${imgDir}/picture_${imageIndex}.png`,
    '-vf',
    `scale=${imageWidth}:-1`,
    '-sws_flags',
    'gauss',
    `${resizedDir}/picture_${imageIndex}.png`,
    '-y',
  ],
  {
    stdio: 'inherit',
  }
);
