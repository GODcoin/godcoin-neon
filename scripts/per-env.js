const spawnSync = require('child_process').spawnSync;

const env = Object.assign({
    NODE_ENV: process.env.NODE_ENV || 'development'
}, process.env);

const command = process.platform !== 'win32' ? 'yarn' : 'yarn.cmd';
const script = env.npm_lifecycle_event + ':' + env.NODE_ENV;
const args = ['run', script].concat(process.argv.slice(2));

const result = spawnSync(command, args, {
    cwd: process.cwd(),
    stdio: 'inherit',
    env
});
process.exit(result.status);
