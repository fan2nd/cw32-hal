'use strict';
const $ = id => document.getElementById(id);
const modes = ['—','位置','速度','力矩','MIT'];
// Values in the editor are in display units; scale converts to wire integers.
const spec = {
  angle:['目标位置 °',-360000,360000,10,0], rpm:['目标速度 rpm',-600,600,1,0],
  iq:['给定 Iq 计数',-200,200,1,0], kp:['Kp · Iq/°',0,10,100,0], kd:['Kd · Iq/rpm',0,10,100,0],
  maxRpm:['速度上限 rpm',1,600,1,600], maxIq:['Iq 上限 计数',1,200,1,200],
  posGain:['位置增益 rpm/°',1,64,1,4], speedKp:['速度 Kp ×64',0,512,1,125],
  speedKi:['速度 Ki ×256',0,128,1,6]
};
const keys=Object.keys(spec);
const draft=Object.fromEntries(keys.map(k=>[k,spec[k][4]]));
const visible=[[],['angle'],['rpm'],['iq'],['angle','rpm','kp','kd','iq']];
let waveView=false;
let edits=0, tuning=false, dirty=false, session=null, connecting=false, serialNumber=0;
function render(){
  $('editor').hidden=waveView;
  document.querySelector('.feedback').hidden=waveView;
  $('waves').hidden=$('waveRelease').hidden=!waveView;
  $('waveTab').setAttribute('aria-pressed',String(waveView));
  $('modeRow').hidden=tuning;
  $('control').setAttribute('aria-pressed',String(!tuning&&!waveView));
  $('tuning').setAttribute('aria-pressed',String(tuning&&!waveView));
  $('fields').replaceChildren();
  for(const key of tuning?keys.slice(5):visible[Number($('mode').value)]){
    const [caption,min,max,scale]=spec[key];
    const label=document.createElement('label');
    label.textContent=key==='iq' && $('mode').value==='4'?'前馈 Iq 计数':caption;
    const input=document.createElement('input');
    Object.assign(input,{id:'field-'+key,type:'number',min,max,step:1/scale,value:draft[key],required:true});
    input.oninput=()=>{draft[key]=input.value;markDirty();};
    label.append(input);$('fields').append(label);
  }
}
function markDirty(){edits++;dirty=true;$('notice').textContent='有未应用的修改';}
function view(name){
  const wasWave=waveView;
  waveView=name==='wave';tuning=name==='tuning';
  if(wasWave!==waveView)scope.reset();
  render();if(session?.ready)void pump(session);
}
$('tuning').onclick=()=>view('tuning');
$('control').onclick=()=>view('control');
$('waveTab').onclick=()=>view('wave');
$('mode').onchange=()=>{markDirty();render();};
function warn(message){$('reason').textContent=message;if(!$('error').open)$('error').showModal();}
function crc(bytes){
  let c=65535;
  for(const b of bytes){c^=b<<8;for(let i=0;i<8;i++)c=((c&32768)?(c<<1)^0x1021:c<<1)&65535;}
  return c;
}
function packet(op,seq){
  const b=new Uint8Array(64),v=new DataView(b.buffer);
  b.set([83,67,op,seq]);b[4]=Number($('mode').value);
  if(op===2){
    const active=visible[b[4]];
    for(const [i,k] of keys.entries()){
      const [,min,max,scale]=spec[k];
      const raw=i<5&&!active.includes(k)?0:draft[k];
      const n=Number(raw),w=Math.round(n*scale);
      const bound=k==='rpm'?Number(draft.maxRpm):k==='iq'?Number(draft.maxIq):Infinity;
      if(raw===''||!Number.isFinite(n)||n<min||n>max||Math.abs(n)>bound||Math.abs(w-n*scale)>1e-7){
        tuning=i>=5;render();$('field-'+k)?.focus();
        if(Number.isFinite(n)&&Math.abs(n)>bound){
          const limit=k==='rpm'?'速度上限':'Iq 上限';
          throw Error(spec[k][0]+' 给定为 '+raw+'，当前「调参」中的'+limit+'为 '+bound+'。');
        }
        throw Error(spec[k][0]+' 允许 '+min+' 至 '+max+'，步长 '+(1/scale)+'。');
      }
      if(i===0)v.setInt32(5,w,true);else v.setInt16(9+(i-1)*2,w,true);
    }
  }
  v.setUint16(30,crc(b.subarray(0,30)),true);return b;
}
function feedback(b,initial){
  const v=new DataView(b.buffer,b.byteOffset,b.byteLength),flags=b[27];
  $('deviceMode').textContent=modes[b[4]]||'未知';
  $('bridge').textContent=flags&2?'MOE 已开启':'已释放';
  $('alarm').textContent=flags&4?'驱动故障':flags&8?'心跳超时':'';
  $('apply').textContent=flags&1 && !(flags&4)?'应用更新':'应用并使能';
  $('angle').textContent=(v.getInt32(28,true)/10).toFixed(1);
  $('rpm').textContent=v.getInt16(32,true);$('iq').textContent=flags&2?v.getInt16(34,true):'—';
  if(initial){
    $('mode').value=b[4];
    keys.forEach((k,i)=>{draft[k]=(i===0?v.getInt32(5,true):v.getInt16(9+(i-1)*2,true))/spec[k][3];});
    dirty=false;render();
  }
}
async function receive(s){
  let bytes=[];
  try{
    while(session===s){
      const {value,done}=await s.reader.read();if(done)break;
      bytes.push(...value);
      let offset=0;
      while(bytes.length-offset>=2){
        const kind=bytes[offset+1];
        if(bytes[offset]!==83||(kind!==82&&kind!==80)){offset++;continue;}
        const size=kind===82?48:236;
        if(bytes.length-offset<size)break;
        const b=Uint8Array.from(bytes.slice(offset,offset+size));
        if(crc(b.subarray(0,size-2))!==(b[size-2]+256*b[size-1])){offset++;continue;}
        offset+=size;
        if(kind===80){if(waveView)scope.ingest(b);}
        else if(s.pending?.seq===b[3])s.pending.resolve(b);
      }
      bytes=bytes.slice(offset);
    }
  }catch(e){s.failure=e;}finally{if(!s.closing)void close(s,s.failure?.message||'串口已断开');}
}
async function exchange(s,op,initial=false,prepared=null){
  try { return await exchangeOnce(s,op,initial,prepared); }
  catch(error){
    // A truncated reply has no valid CRC. Retry only non-driving operations,
    // with a fresh sequence, within the 250 ms firmware lease.
    if(error.name!=='TimeoutError'||op===2||session!==s)throw error;
    return exchangeOnce(s,op,initial,prepared);
  }
}
async function exchangeOnce(s,op,initial,prepared){
  if(s.pending||session!==s)throw Error('串口正在处理上一帧');
  const seq=(++serialNumber)&255,b=prepared||packet(op,seq);
  b[3]=seq;new DataView(b.buffer).setUint16(30,crc(b.subarray(0,30)),true);
  let resolve,reject;
  const response=new Promise((ok,no)=>{resolve=ok;reject=no;});
  s.pending={seq,resolve,reject};
  const timer=setTimeout(()=>reject(new DOMException('设备应答超时','TimeoutError')),90);
  try{
    const [,reply]=await Promise.all([s.writer.write(b),response]);
    if(session!==s)throw Error('连接已关闭');
    if(op===1 && s.streaming && !(reply[27]&16)){
      s.streaming=false;
      scope.reset();
    }
    feedback(reply,initial);
    return reply[2]===0;
  }finally{clearTimeout(timer);s.pending=null;}
}
async function close(s,message){
  if(s.closing)return;
  scope.disconnect();
  s.closing=true;clearInterval(s.timer);session=null;
  s.pending?.reject(Error('连接已关闭'));
  $('apply').disabled=$('waveRelease').disabled=$('release').disabled=$('connect').disabled=true;
  $('connection').textContent='串口未连接';$('bridge').textContent='未知';$('deviceMode').textContent='—';$('alarm').textContent='';
  $('notice').textContent=message+' · 最后反馈';$('connect').textContent='连接串口';
  try{await s.reader.cancel();}catch{}
  s.reader.releaseLock();
  try{await s.writer.abort();}catch{}
  s.writer.releaseLock();try{await s.port.close();}catch{}
  $('connect').disabled=false;
}
async function pump(s){
  if(session!==s||s.pending||s.closing)return;
  // Reserve heartbeats under repeated Apply clicks. Release always takes priority.
  const due=performance.now()-(s.lastHeartbeat||0)>=48;
  const request=due && s.next?.op===2 ? null : s.next;
  const change=!!s.streaming!==waveView && !s.waveRejected;
  const op=request?.op||(!due && change?(waveView?7:6):1);if(request)s.next=null;
  if(op===1)s.lastHeartbeat=performance.now();
  try{
    const ok=await exchange(s,op,false,request?.frame);
    if(op===2&&ok&&request.edits===edits){$('notice').textContent='设备已接受指令';dirty=false;}
    if(op===3&&ok)$('notice').textContent=dirty?'已释放 · 有未应用的修改':'已释放';
    if(op===7||op===6){if(ok)s.streaming=op===7;else s.waveRejected=true;}
    if(!ok)warn('设备拒绝指令，请检查参数、心跳或先释放故障。');
  }catch(e){await close(s,e.message);}
  if(s.next||(!s.waveRejected&&!!s.streaming!==waveView))void pump(s);
}
$('editor').onsubmit=e=>{
  e.preventDefault();if(!session)return;
  let frame;try{frame=packet(2,0);}catch(err){warn(err.message);return;}
  session.next={op:2,frame,edits};void pump(session);
};
$('release').onclick=()=>{if(session){session.next={op:3};void pump(session);}};
$('waveRelease').onclick=()=>$('release').click();
$('connect').onclick=async()=>{
  if(session){await close(session,'已断开');return;}
  if(connecting)return;
  connecting=true;$('connect').disabled=true;
  let port,s;
  try{
    if(!navigator.serial)throw Error('请使用支持 Web Serial 的桌面 Edge 或 Chrome 打开此页面。');
    port=await navigator.serial.requestPort();
    await port.open({baudRate:1000000,dataBits:8,stopBits:1,parity:'none',flowControl:'none'});
    s={port,reader:port.readable.getReader(),writer:port.writable.getWriter(),next:0};
    session=s;void receive(s);
    // A fresh session releases any surviving command before renewing its lease.
    await exchange(s,4,true);
    await exchange(s,1);
    $('connection').textContent='已连接 · 1 Mbps';$('connect').textContent='断开';
    $('apply').disabled=$('waveRelease').disabled=$('release').disabled=false;$('notice').textContent='设备参数已回读';
    s.ready=true;s.lastHeartbeat=performance.now();scope.reset();
    if(waveView)void pump(s);
    s.timer=setInterval(()=>void pump(s),50);
  }catch(e){if(s)await close(s,e.message);else if(port)try{await port.close();}catch{}
    if(e.name!=='NotFoundError')warn(e.message);
  }finally{connecting=false;$('connect').disabled=false;}
};
render();
