import{S as re,i as ae,s as ie,ab as I,e as g,f as O,z as j,l as k,E as V,v as y,ac as X,g as A,n as w,a6 as de,L as $e,c as M,a as N,t as C,b as S,d as q,o as ge,H as be,h as U,q as fe,r as ue,u as pe,M as he,x as Z,y as ce,A as _e,_ as we,$ as ke,ad as Y,C as ye,j as D,k as ve,p as x,ae as W,a9 as Ce,aa as E,a3 as J,a2 as Se,a0 as P,a4 as Fe,a5 as Le}from"./Legend-8c5ea55c.js";import{_ as je,J as Me,C as Ne,G as qe}from"./CostFunction-1c17c3c6.js";function ee(s,t,n){const e=s.slice();return e[2]=t[n],e}function te(s,t,n){const e=s.slice();return e[5]=t[n],e}function ne(s){let t,n;return{c(){t=g("span"),n=A(` 
    `),j(t,"background",s[5]),j(t,"width","100%"),j(t,"border","1px solid black")},m(e,l){k(e,t,l),w(t,n)},p(e,l){l&1&&j(t,"background",e[5])},d(e){e&&y(t)}}}function le(s){let t,n=s[2].toFixed(1)+"",e;return{c(){t=g("span"),e=A(n)},m(l,o){k(l,t,o),w(t,e)},p(l,o){o&2&&n!==(n=l[2].toFixed(1)+"")&&de(e,n)},d(l){l&&y(t)}}}function Oe(s){let t,n,e,l=I(s[0]),o=[];for(let r=0;r<l.length;r+=1)o[r]=ne(te(s,l,r));let f=I(s[1]),i=[];for(let r=0;r<f.length;r+=1)i[r]=le(ee(s,f,r));return{c(){t=g("div");for(let r=0;r<o.length;r+=1)o[r].c();n=O(),e=g("div");for(let r=0;r<i.length;r+=1)i[r].c();j(t,"display","flex"),j(e,"display","flex"),j(e,"justify-content","space-between")},m(r,p){k(r,t,p);for(let a=0;a<o.length;a+=1)o[a]&&o[a].m(t,null);k(r,n,p),k(r,e,p);for(let a=0;a<i.length;a+=1)i[a]&&i[a].m(e,null)},p(r,[p]){if(p&1){l=I(r[0]);let a;for(a=0;a<l.length;a+=1){const c=te(r,l,a);o[a]?o[a].p(c,p):(o[a]=ne(c),o[a].c(),o[a].m(t,null))}for(;a<o.length;a+=1)o[a].d(1);o.length=l.length}if(p&2){f=I(r[1]);let a;for(a=0;a<f.length;a+=1){const c=ee(r,f,a);i[a]?i[a].p(c,p):(i[a]=le(c),i[a].c(),i[a].m(e,null))}for(;a<i.length;a+=1)i[a].d(1);i.length=f.length}},i:V,o:V,d(r){r&&(y(t),y(n),y(e)),X(o,r),X(i,r)}}}function Be(s,t,n){let{colorScale:e}=t,{limits:l}=t;return s.$$set=o=>{"colorScale"in o&&n(0,e=o.colorScale),"limits"in o&&n(1,l=o.limits)},[e,l]}class Ee extends re{constructor(t){super(),ae(this,t,Be,Oe,ie,{colorScale:0,limits:1})}}function se(s){let t,n,e,l,o,f,i,r,p,a,c,b,F,B,_,d,L,T,H,G;const m=[Te,Je],h=[];function $(u,v){return u[4]=="lts"?0:1}c=$(s),b=h[c]=m[c](s);function me(u){s[15](u)}let K={};return s[0]!==void 0&&(K.cost=s[0]),d=new Ne({props:K}),Z.push(()=>ce(d,"cost",me)),{c(){t=g("hr"),n=O(),e=g("div"),l=g("label"),o=A(`Color edges by:
          `),f=g("select"),i=g("option"),i.textContent="LTS",r=g("option"),r.textContent="Edge cost (relative to length)",p=g("option"),p.textContent="Nearby amenities",a=O(),b.c(),F=O(),B=g("hr"),_=O(),M(d.$$.fragment),i.__value="lts",D(i,i.__value),r.__value="cost",D(r,r.__value),p.__value="nearby_amenities",D(p,p.__value),s[4]===void 0&&ve(()=>s[14].call(f))},m(u,v){k(u,t,v),k(u,n,v),k(u,e,v),w(e,l),w(l,o),w(l,f),w(f,i),w(f,r),w(f,p),x(f,s[4],!0),k(u,a,v),h[c].m(u,v),k(u,F,v),k(u,B,v),k(u,_,v),N(d,u,v),T=!0,H||(G=fe(f,"change",s[14]),H=!0)},p(u,v){v&16&&x(f,u[4]);let z=c;c=$(u),c===z?h[c].p(u,v):(ue(),S(h[z],1,1,()=>{h[z]=null}),pe(),b=h[c],b?b.p(u,v):(b=h[c]=m[c](u),b.c()),C(b,1),b.m(F.parentNode,F));const Q={};!L&&v&1&&(L=!0,Q.cost=u[0],_e(()=>L=!1)),d.$set(Q)},i(u){T||(C(b),C(d.$$.fragment,u),T=!0)},o(u){S(b),S(d.$$.fragment,u),T=!1},d(u){u&&(y(t),y(n),y(e),y(a),y(F),y(B),y(_)),h[c].d(u),q(d,u),H=!1,G()}}}function Je(s){let t,n;return t=new Ee({props:{colorScale:W,limits:R(s[4],s[6],s[7])}}),{c(){M(t.$$.fragment)},m(e,l){N(t,e,l),n=!0},p(e,l){const o={};l&208&&(o.limits=R(e[4],e[6],e[7])),t.$set(o)},i(e){n||(C(t.$$.fragment,e),n=!0)},o(e){S(t.$$.fragment,e),n=!1},d(e){q(t,e)}}}function Te(s){let t,n,e,l;return t=new Ce({props:{rows:[[`${E.lts1}: ${s[5][1].toFixed(0)}% of roads by distance`,J.lts1],[`${E.lts2}: ${s[5][2].toFixed(0)}%`,J.lts2],[`${E.lts3}: ${s[5][3].toFixed(0)}%`,J.lts3],[`${E.lts4}: ${s[5][4].toFixed(0)}%`,J.lts4]]}}),{c(){M(t.$$.fragment),n=O(),e=g("p"),e.innerHTML='Note: LTS model from <a href="https://github.com/BikeOttawa/stressmodel/blob/master/stressmodel.js" target="_blank">BikeOttawa</a>'},m(o,f){N(t,o,f),k(o,n,f),k(o,e,f),l=!0},p(o,f){const i={};f&32&&(i.rows=[[`${E.lts1}: ${o[5][1].toFixed(0)}% of roads by distance`,J.lts1],[`${E.lts2}: ${o[5][2].toFixed(0)}%`,J.lts2],[`${E.lts3}: ${o[5][3].toFixed(0)}%`,J.lts3],[`${E.lts4}: ${o[5][4].toFixed(0)}%`,J.lts4]]),t.$set(i)},i(o){l||(C(t.$$.fragment,o),l=!0)},o(o){S(t.$$.fragment,o),l=!1},d(o){o&&(y(n),y(e)),q(t,o)}}}function He(s){let t,n,e,l,o,f,i,r,p,a,c,b,F,B;n=new be({props:{app:"costs"}});let _=s[2]&&se(s);return{c(){t=g("div"),M(n.$$.fragment),e=O(),l=g("label"),o=A("Open a "),f=g("i"),f.textContent=".bin",i=A(" network file or an "),r=g("i"),r.textContent=".osm.pbf",p=O(),a=g("input"),c=O(),_&&_.c(),U(a,"type","file"),U(t,"slot","left")},m(d,L){k(d,t,L),N(n,t,null),w(t,e),w(t,l),w(l,o),w(l,f),w(l,i),w(l,r),w(l,p),w(l,a),s[13](a),w(t,c),_&&_.m(t,null),b=!0,F||(B=fe(a,"change",s[9]),F=!0)},p(d,L){d[2]?_?(_.p(d,L),L&4&&C(_,1)):(_=se(d),_.c(),C(_,1),_.m(t,null)):_&&(ue(),S(_,1,1,()=>{_=null}),pe())},i(d){b||(C(n.$$.fragment,d),C(_),b=!0)},o(d){S(n.$$.fragment,d),S(_),b=!1},d(d){d&&y(t),q(n),s[13](null),_&&_.d(),F=!1,B()}}}function Ae(s){let t,n;return t=new Le({props:{properties:s[18][0].properties}}),{c(){M(t.$$.fragment)},m(e,l){N(t,e,l),n=!0},p(e,l){const o={};l&262144&&(o.properties=e[18][0].properties),t.$set(o)},i(e){n||(C(t.$$.fragment,e),n=!0)},o(e){S(t.$$.fragment,e),n=!1},d(e){q(t,e)}}}function Ge(s){let t,n;return t=new Fe({props:{openOn:"hover",$$slots:{default:[Ae,({features:e})=>({18:e}),({features:e})=>e?262144:0]},$$scope:{ctx:s}}}),{c(){M(t.$$.fragment)},m(e,l){N(t,e,l),n=!0},p(e,l){const o={};l&786432&&(o.$$scope={dirty:l,ctx:e}),t.$set(o)},i(e){n||(C(t.$$.fragment,e),n=!0)},o(e){S(t.$$.fragment,e),n=!1},d(e){q(t,e)}}}function Ie(s){let t,n;return t=new Se({props:{manageHoverState:!0,hoverCursor:"pointer",paint:{"line-width":5,"line-color":s[10](s[4],s[6],s[7]),"line-opacity":s[4]=="nearby_amenities"?["case",["==",0,["get","nearby_amenities"]],0,P(1,.5)]:P(1,.5)},beforeId:"Road labels",$$slots:{default:[Ge]},$$scope:{ctx:s}}}),t.$on("click",s[11]),{c(){M(t.$$.fragment)},m(e,l){N(t,e,l),n=!0},p(e,l){const o={};l&208&&(o.paint={"line-width":5,"line-color":e[10](e[4],e[6],e[7]),"line-opacity":e[4]=="nearby_amenities"?["case",["==",0,["get","nearby_amenities"]],0,P(1,.5)]:P(1,.5)}),l&524288&&(o.$$scope={dirty:l,ctx:e}),t.$set(o)},i(e){n||(C(t.$$.fragment,e),n=!0)},o(e){S(t.$$.fragment,e),n=!1},d(e){q(t,e)}}}function Pe(s){let t,n;return t=new qe({props:{data:s[3],$$slots:{default:[Ie]},$$scope:{ctx:s}}}),{c(){M(t.$$.fragment)},m(e,l){N(t,e,l),n=!0},p(e,l){const o={};l&8&&(o.data=e[3]),l&524496&&(o.$$scope={dirty:l,ctx:e}),t.$set(o)},i(e){n||(C(t.$$.fragment,e),n=!0)},o(e){S(t.$$.fragment,e),n=!1},d(e){q(t,e)}}}function Re(s){let t,n,e,l;function o(i){s[12](i)}let f={style:"https://api.maptiler.com/maps/dataviz/style.json?key=MZEJTanw3WpxRvt7qDfo",standardControls:!0,hash:!0,$$slots:{default:[Pe]},$$scope:{ctx:s}};return s[1]!==void 0&&(f.map=s[1]),n=new he({props:f}),Z.push(()=>ce(n,"map",o)),{c(){t=g("div"),M(n.$$.fragment),U(t,"slot","main"),j(t,"position","relative"),j(t,"width","100%"),j(t,"height","100vh")},m(i,r){k(i,t,r),N(n,t,null),l=!0},p(i,r){const p={};r&524504&&(p.$$scope={dirty:r,ctx:i}),!e&&r&2&&(e=!0,p.map=i[1],_e(()=>e=!1)),n.$set(p)},i(i){l||(C(n.$$.fragment,i),l=!0)},o(i){S(n.$$.fragment,i),l=!1},d(i){i&&y(t),q(n)}}}function ze(s){let t,n;return t=new $e({props:{$$slots:{main:[Re],left:[He]},$$scope:{ctx:s}}}),{c(){M(t.$$.fragment)},m(e,l){N(t,e,l),n=!0},p(e,[l]){const o={};l&524799&&(o.$$scope={dirty:l,ctx:e}),t.$set(o)},i(e){n||(C(t.$$.fragment,e),n=!0)},o(e){S(t.$$.fragment,e),n=!1},d(e){q(t,e)}}}function De(s){let t=s.properties.way;window.open(`http://openstreetmap.org/way/${t}`,"_blank")}function R(s,t,n){if(s=="lts")return null;if(s=="cost")return oe(0,t);if(s=="nearby_amenities")return oe(0,n)}function oe(s,t){let n=[],e=(t-s)/5;for(let l=0;l<6;l++)n.push(s+l*e);return n}function Ue(s,t,n){ge(async()=>{await je(),await we()});let e,l,o={type:"FeatureCollection",features:[]},f="Distance",i="cost",r=[0,0,0,0,0],p=1,a=1,c;async function b(m){try{let h=await c.files[0].arrayBuffer();n(2,l=new Me(new Uint8Array(h)));let $=l.getBounds();e.fitBounds([[$[0],$[1]],[$[2],$[3]]],{padding:20,animate:!1}),F()}catch(h){window.alert(`Problem loading network file: ${h}`)}}function F(){n(3,o=JSON.parse(l.debugNetwork()));let m=0,h=[0,0,0,0,0];n(6,p=0),n(7,a=0);for(let $ of o.features)n(7,a=Math.max(a,$.properties.nearby_amenities)),$.properties.length&&(n(6,p=Math.max(p,$.properties.cost/$.properties.length)),m+=$.properties.length,h[$.properties.lts]+=$.properties.length);n(5,r=h.map($=>$/m*100))}function B(m,h,$){if(m=="lts")return ke;if(m=="cost")return Y(["/",["get","cost"],["get","length"]],R(m,h,$),W);if(m=="nearby_amenities")return Y(["get","nearby_amenities"],R(m,h,$),W)}function _(m){l&&(l.updateCostFunction(m),F())}const d=m=>De(m.detail.features[0]);function L(m){e=m,n(1,e)}function T(m){Z[m?"unshift":"push"](()=>{c=m,n(8,c)})}function H(){i=ye(this),n(4,i)}function G(m){f=m,n(0,f)}return s.$$.update=()=>{s.$$.dirty&1&&_(f)},[f,e,l,o,i,r,p,a,c,b,B,d,L,T,H,G]}class We extends re{constructor(t){super(),ae(this,t,Ue,ze,ie,{})}}new We({target:document.getElementById("app")});
