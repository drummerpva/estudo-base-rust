import init, {game_start, game_tick} from './pkg/first_wasm.js'

async function bootstrap(){
  await init();
  game_start();
  let game_ctx = {
    last_tick : performance.now()
  }

  window.requestAnimationFrame((timestamp)=>{
    loop(timestamp, game_ctx)
  })
}

function loop(timestam, game_ctx){
  let elapsed = timestam - game_ctx.last_tick;
  game_tick(elapsed);
  draw(game_ctx);

  game_ctx.last_tick = performance.now();
  window.requestAnimationFrame((timestamp)=>{
    loop(timestamp, game_ctx);
  })
}
function draw(game_ctx){
  
}
bootstrap()