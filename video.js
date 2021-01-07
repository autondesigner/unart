const fs = require('fs');

function generateVideo(path, width) {
  let videoName = path;
  const imgDir = path;
  let videoWidth = width;

  const child_process = require('child_process');

  child_process.spawnSync(
    'ffmpeg',
    [
      '-framerate',
      '60',
      '-i',
      `render/${imgDir}/picture_%d.png`,
      '-vf',
      `scale=${videoWidth}:-1`,
      '-sws_flags',
      'neighbor',
      '-b:v',
      '32M',
      '-vcodec',
      'h264',
      `render/videos/${videoName}.mov`,
      '-y',
    ],
    {
      stdio: 'inherit',
    }
  );
}
if (!fs.existsSync('render/videos')) {
  fs.mkdirSync('render/videos');
}

let version = 'a0';
if (process.argv.length > 2) {
  version = process.argv[2];
}
let width = 2048;
if (process.argv.length > 3) {
  width = process.argv[3];
}
generateVideo(version, width);
