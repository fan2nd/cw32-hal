'use strict';
const scope=(()=>{
  const get=id=>document.getElementById(id);
  const capacity=2000, colors=['#0754a1','#a3440b','#16705a','#674ba3','#11645c'];
  const names=['Ia','Ib','Ic*','Id','Iq'];
  let live=new Array(capacity), end=0, previous=null, lost=0, frozen=null, cursor=null, scheduled=false;
  function reset(){
    live=new Array(capacity);end=0;previous=null;lost=0;frozen=null;cursor=null;
    get('span').value='2000';get('pan').value='0';schedule();
  }
  function ingest(bytes){
    const v=new DataView(bytes.buffer,bytes.byteOffset,bytes.byteLength);
    const first=v.getUint32(2,true), mask=v.getUint16(6,true);
    const gap=previous===null?0:(first-previous)>>>0;
    if(gap>0x7fffffff)return;
    lost+=gap;
    if(gap>=capacity)live.fill(undefined);
    else for(let i=0;i<gap;i++)live[(end+i)%capacity]=undefined;
    end+=gap;
    for(let i=0;i<16;i++){
      const p=10+i*14,a=v.getInt16(p,true),b=v.getInt16(p+2,true);
      live[end++%capacity]=mask&(1<<i)?[a,b,-a-b,v.getInt16(p+4,true),v.getInt16(p+6,true),v.getUint16(p+8,true),v.getUint16(p+10,true),v.getUint16(p+12,true),v.getUint16(8,true)]:undefined;
    }
    previous=(first+16)>>>0;
    if(!frozen)schedule();
  }
  function at(index){return index<0?undefined:live[index%capacity];}
  function schedule(){if(!scheduled){scheduled=true;requestAnimationFrame(()=>{scheduled=false;draw();});}}
  function draw(){
    if(get('waves').hidden)return;
    const n=frozen?Number(get('span').value):capacity;
    const offset=frozen?Number(get('pan').value):0;
    const point=i=>frozen?frozen[offset+i]:at(end-capacity+i);
    const ix=cursor===null?n-1:Math.round(cursor*(n-1));
    const sample=point(ix);
    get('freeze').textContent=frozen?'继续':'冻结';
    get('span').hidden=get('pan').hidden=!frozen;
    get('pan').disabled=n===capacity;
    get('waveState').textContent=frozen?'已冻结':previous===null?'等待波形':'实时 · 最近 1 s';
    get('loss').textContent=lost?'缺样 '+lost:'';
    for(const [id,legend,lines] of [['phases','phaseLegend',[0,1,2]],['pwm','pwmLegend',[5,6,7]],['dq','dqLegend',[3,4]]]){
      const canvas=get(id),w=canvas.clientWidth,h=canvas.clientHeight;
      const r=devicePixelRatio||1;
      if(canvas.width!==Math.round(w*r)||canvas.height!==Math.round(h*r)){canvas.width=Math.round(w*r);canvas.height=Math.round(h*r);}
      const c=canvas.getContext('2d');c.setTransform(r,0,0,r,0,0);c.clearRect(0,0,w,h);
      const pwm=id==='pwm';
      const value=(p,k)=>pwm?100*p[k]/p[8]:p[k];
      const center=pwm?50:0;
      let max=pwm?1:10;
      for(let i=0;i<n;i++){const p=point(i);if(p)for(const k of lines)max=Math.max(max,Math.abs(value(p,k)-center));}
      const unit=10**Math.floor(Math.log10(max));max=Math.ceil(max/unit)*unit;
      if(pwm)max=Math.min(50,max);
      const x=i=>40+i*(w-42)/(n-1), y=v=>7+(1-(v-center)/max)*(h-14)/2;
      c.font='11px monospace';c.fillStyle='#425960';c.strokeStyle='#c2d2d4';c.setLineDash([]);
      for(const v of [center+max,center,center-max]){c.fillText(String(v),1,y(v)+3);c.beginPath();c.moveTo(40,y(v));c.lineTo(w,y(v));c.stroke();}
      for(const k of lines){
        c.strokeStyle=colors[pwm?k-5:k];c.setLineDash(k===2?[4,3]:[]);c.beginPath();
        let pen=false;
        for(let i=0;i<n;i++){const p=point(i);if(!p){pen=false;continue;}if(pen)c.lineTo(x(i),y(value(p,k)));else c.moveTo(x(i),y(value(p,k)));pen=true;}
        c.stroke();
      }
      c.setLineDash([]);
      if(cursor!==null){c.strokeStyle='#50646c';c.beginPath();c.moveTo(x(ix),0);c.lineTo(x(ix),h);c.stroke();}
      get(legend).innerHTML=lines.map(k=>{
        const label=pwm?'CCR'+(k-4):k===2?'Ic(重建)':names[k];
        const readout=!sample?'—':pwm?sample[k]+' / '+value(sample,k).toFixed(1)+'%':sample[k];
        return '<i style="color:'+colors[pwm?k-5:k]+'">'+label+' '+readout+'</i>';
      }).join('');
    }
    const ms=i=>((offset+i-capacity+1)/2)+' ms';
    get('timeAxis').innerHTML=[0,Math.floor((n-1)/2),n-1].map(i=>'<span>'+ms(i)+'</span>').join('');
  }
  get('freeze').onclick=()=>{
    if(frozen){frozen=null;}else{frozen=Array.from({length:capacity},(_,i)=>at(end-capacity+i));get('span').value='2000';get('pan').max=0;get('pan').value=0;}
    schedule();
  };
  get('span').onchange=()=>{const n=Number(get('span').value);const pan=get('pan');const center=Number(pan.value)+(capacity-Number(pan.max))/2;pan.max=capacity-n;pan.value=Math.round(Math.max(0,center-n/2));schedule();};
  get('pan').oninput=schedule;
  for(const id of ['phases','pwm','dq']){
    get(id).onpointermove=e=>{const rect=e.currentTarget.getBoundingClientRect();cursor=Math.max(0,Math.min(1,(e.clientX-rect.left-40)/(rect.width-42)));schedule();};
    get(id).onpointerleave=()=>{cursor=null;schedule();};
  }
  window.addEventListener('resize',schedule);
  return {reset,ingest,disconnect:()=>{previous=null;schedule();}};
})();
