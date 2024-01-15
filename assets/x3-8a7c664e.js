import{S as ge,i as $e,s as be,ab as K,e as $,f as N,z as J,l as k,E as oe,v as y,ac as re,g as W,n as b,W as he,L as ye,c as j,a as B,t as C,b as F,d as q,o as ve,H as Se,x as Y,y as le,h as Q,q as te,A as se,r as we,u as ke,M as Ce,_ as Fe,a0 as Le,a4 as A,ad as ae,C as Me,j as ee,k as Oe,p as ie,w as Ne,ae as ne,a1 as je,a9 as Be,aa as I,a3 as qe,a5 as Ee,a6 as Te}from"./Legend-814dd8c6.js";import{L as Je,C as Pe,O as Ae,_ as He,J as De,a as Ge,G as Ie}from"./OverpassSelector-55493a61.js";function fe(l,e,n){const t=l.slice();return t[2]=e[n],t}function pe(l,e,n){const t=l.slice();return t[5]=e[n],t}function ue(l){let e,n;return{c(){e=$("span"),n=W(` 
    `),J(e,"background",l[5]),J(e,"width","100%"),J(e,"border","1px solid black")},m(t,s){k(t,e,s),b(e,n)},p(t,s){s&1&&J(e,"background",t[5])},d(t){t&&y(e)}}}function ce(l){let e,n=l[2].toFixed(1)+"",t;return{c(){e=$("span"),t=W(n)},m(s,o){k(s,e,o),b(e,t)},p(s,o){o&2&&n!==(n=s[2].toFixed(1)+"")&&he(t,n)},d(s){s&&y(e)}}}function Re(l){let e,n,t,s=K(l[0]),o=[];for(let r=0;r<s.length;r+=1)o[r]=ue(pe(l,s,r));let f=K(l[1]),i=[];for(let r=0;r<f.length;r+=1)i[r]=ce(fe(l,f,r));return{c(){e=$("div");for(let r=0;r<o.length;r+=1)o[r].c();n=N(),t=$("div");for(let r=0;r<i.length;r+=1)i[r].c();J(e,"display","flex"),J(t,"display","flex"),J(t,"justify-content","space-between")},m(r,m){k(r,e,m);for(let a=0;a<o.length;a+=1)o[a]&&o[a].m(e,null);k(r,n,m),k(r,t,m);for(let a=0;a<i.length;a+=1)i[a]&&i[a].m(t,null)},p(r,[m]){if(m&1){s=K(r[0]);let a;for(a=0;a<s.length;a+=1){const d=pe(r,s,a);o[a]?o[a].p(d,m):(o[a]=ue(d),o[a].c(),o[a].m(e,null))}for(;a<o.length;a+=1)o[a].d(1);o.length=s.length}if(m&2){f=K(r[1]);let a;for(a=0;a<f.length;a+=1){const d=fe(r,f,a);i[a]?i[a].p(d,m):(i[a]=ce(d),i[a].c(),i[a].m(t,null))}for(;a<i.length;a+=1)i[a].d(1);i.length=f.length}},i:oe,o:oe,d(r){r&&(y(e),y(n),y(t)),re(o,r),re(i,r)}}}function Xe(l,e,n){let{colorScale:t}=e,{limits:s}=e;return l.$$set=o=>{"colorScale"in o&&n(0,t=o.colorScale),"limits"in o&&n(1,s=o.limits)},[t,s]}class ze extends ge{constructor(e){super(),$e(this,e,Xe,Re,be,{colorScale:0,limits:1})}}function _e(l){let e,n;return{c(){e=$("p"),n=W(l[12])},m(t,s){k(t,e,s),b(e,n)},p(t,s){s[0]&4096&&he(n,t[12])},d(t){t&&y(e)}}}function me(l){let e,n,t,s,o,f,i,r,m,a,d,v,h,E,T,M,D,H,R,G,c,g,_,O,X;const z=[Ue,We],P=[];function U(u,w){return u[5]=="lts"?0:1}d=U(l),v=P[d]=z[d](l);function x(u){l[25](u)}let Z={};return l[1]!==void 0&&(Z.cost=l[1]),c=new Ge({props:Z}),Y.push(()=>le(c,"cost",x)),{c(){e=$("hr"),n=N(),t=$("div"),s=$("label"),o=W(`Color edges by:
          `),f=$("select"),i=$("option"),i.textContent="LTS",r=$("option"),r.textContent="Edge cost (relative to length)",m=$("option"),m.textContent="Nearby amenities",a=N(),v.c(),h=N(),E=$("div"),T=$("label"),M=$("input"),D=W(`
          Show cyclists not allowed`),H=N(),R=$("hr"),G=N(),j(c.$$.fragment),i.__value="lts",ee(i,i.__value),r.__value="cost",ee(r,r.__value),m.__value="nearby_amenities",ee(m,m.__value),l[5]===void 0&&Oe(()=>l[23].call(f)),Q(M,"type","checkbox"),J(T,"color",A.lts_not_allowed)},m(u,w){k(u,e,w),k(u,n,w),k(u,t,w),b(t,s),b(s,o),b(s,f),b(f,i),b(f,r),b(f,m),ie(f,l[5],!0),k(u,a,w),P[d].m(u,w),k(u,h,w),k(u,E,w),b(E,T),b(T,M),M.checked=l[6],b(T,D),k(u,H,w),k(u,R,w),k(u,G,w),B(c,u,w),_=!0,O||(X=[te(f,"change",l[23]),te(M,"change",l[24])],O=!0)},p(u,w){w[0]&32&&ie(f,u[5]);let p=d;d=U(u),d===p?P[d].p(u,w):(we(),F(P[p],1,1,()=>{P[p]=null}),ke(),v=P[d],v?v.p(u,w):(v=P[d]=z[d](u),v.c()),C(v,1),v.m(h.parentNode,h)),w[0]&64&&(M.checked=u[6]);const S={};!g&&w[0]&2&&(g=!0,S.cost=u[1],se(()=>g=!1)),c.$set(S)},i(u){_||(C(v),C(c.$$.fragment,u),_=!0)},o(u){F(v),F(c.$$.fragment,u),_=!1},d(u){u&&(y(e),y(n),y(t),y(a),y(h),y(E),y(H),y(R),y(G)),P[d].d(u),q(c,u),O=!1,Ne(X)}}}function We(l){let e,n;return e=new ze({props:{colorScale:ne,limits:V(l[5],l[9],l[10])}}),{c(){j(e.$$.fragment)},m(t,s){B(e,t,s),n=!0},p(t,s){const o={};s[0]&1568&&(o.limits=V(t[5],t[9],t[10])),e.$set(o)},i(t){n||(C(e.$$.fragment,t),n=!0)},o(t){F(e.$$.fragment,t),n=!1},d(t){q(e,t)}}}function Ue(l){let e,n,t,s;return e=new Be({props:{rows:[[`${I.lts1}: ${l[8][1].toFixed(0)}% of roads by distance`,A.lts1],[`${I.lts2}: ${l[8][2].toFixed(0)}%`,A.lts2],[`${I.lts3}: ${l[8][3].toFixed(0)}%`,A.lts3],[`${I.lts4}: ${l[8][4].toFixed(0)}%`,A.lts4]]}}),{c(){j(e.$$.fragment),n=N(),t=$("p"),t.innerHTML='Note: LTS model from <a href="https://github.com/BikeOttawa/stressmodel/blob/master/stressmodel.js" target="_blank">BikeOttawa</a>'},m(o,f){B(e,o,f),k(o,n,f),k(o,t,f),s=!0},p(o,f){const i={};f[0]&256&&(i.rows=[[`${I.lts1}: ${o[8][1].toFixed(0)}% of roads by distance`,A.lts1],[`${I.lts2}: ${o[8][2].toFixed(0)}%`,A.lts2],[`${I.lts3}: ${o[8][3].toFixed(0)}%`,A.lts3],[`${I.lts4}: ${o[8][4].toFixed(0)}%`,A.lts4]]),e.$set(i)},i(o){s||(C(e.$$.fragment,o),s=!0)},o(o){F(e.$$.fragment,o),s=!1},d(o){o&&(y(n),y(t)),q(e,o)}}}function Ze(l){let e,n,t,s,o,f,i,r,m,a,d,v,h,E,T,M,D,H;n=new Se({props:{app:"costs"}});function R(_){l[20](_)}let G={};l[0]!==void 0&&(G.example=l[0]),a=new Pe({props:G}),Y.push(()=>le(a,"example",R)),h=new Ae({props:{map:l[2]}}),h.$on("gotXml",l[16]),h.$on("loading",l[21]),h.$on("error",l[22]);let c=l[12]&&_e(l),g=l[3]&&me(l);return{c(){e=$("div"),j(n.$$.fragment),t=N(),s=$("label"),o=W("Open an "),f=$("i"),f.textContent=".osm.pbf",i=W(` file
      `),r=$("input"),m=N(),j(a.$$.fragment),v=N(),j(h.$$.fragment),E=N(),c&&c.c(),T=N(),g&&g.c(),Q(r,"type","file"),Q(e,"slot","left")},m(_,O){k(_,e,O),B(n,e,null),b(e,t),b(e,s),b(s,o),b(s,f),b(s,i),b(s,r),l[19](r),b(e,m),B(a,e,null),b(e,v),B(h,e,null),b(e,E),c&&c.m(e,null),b(e,T),g&&g.m(e,null),M=!0,D||(H=te(r,"change",l[13]),D=!0)},p(_,O){const X={};!d&&O[0]&1&&(d=!0,X.example=_[0],se(()=>d=!1)),a.$set(X);const z={};O[0]&4&&(z.map=_[2]),h.$set(z),_[12]?c?c.p(_,O):(c=_e(_),c.c(),c.m(e,T)):c&&(c.d(1),c=null),_[3]?g?(g.p(_,O),O[0]&8&&C(g,1)):(g=me(_),g.c(),C(g,1),g.m(e,null)):g&&(we(),F(g,1,1,()=>{g=null}),ke())},i(_){M||(C(n.$$.fragment,_),C(a.$$.fragment,_),C(h.$$.fragment,_),C(g),M=!0)},o(_){F(n.$$.fragment,_),F(a.$$.fragment,_),F(h.$$.fragment,_),F(g),M=!1},d(_){_&&y(e),q(n),l[19](null),q(a),q(h),c&&c.d(),g&&g.d(),D=!1,H()}}}function Ke(l){let e,n;return e=new Te({props:{properties:l[30][0].properties}}),{c(){j(e.$$.fragment)},m(t,s){B(e,t,s),n=!0},p(t,s){const o={};s[0]&1073741824&&(o.properties=t[30][0].properties),e.$set(o)},i(t){n||(C(e.$$.fragment,t),n=!0)},o(t){F(e.$$.fragment,t),n=!1},d(t){q(e,t)}}}function Qe(l){let e,n;return e=new Ee({props:{openOn:"hover",$$slots:{default:[Ke,({features:t})=>({30:t}),({features:t})=>[t?1073741824:0]]},$$scope:{ctx:l}}}),{c(){j(e.$$.fragment)},m(t,s){B(e,t,s),n=!0},p(t,s){const o={};s[0]&1073741824|s[1]&1&&(o.$$scope={dirty:s,ctx:t}),e.$set(o)},i(t){n||(C(e.$$.fragment,t),n=!0)},o(t){F(e.$$.fragment,t),n=!1},d(t){q(e,t)}}}function Ve(l){let e,n;return e=new qe({props:{manageHoverState:!0,hoverCursor:"pointer",paint:{"line-width":5,"line-color":l[14](l[5],l[9],l[10]),"line-opacity":l[15](l[5],l[6])},beforeId:"Road labels",$$slots:{default:[Qe]},$$scope:{ctx:l}}}),e.$on("click",l[17]),{c(){j(e.$$.fragment)},m(t,s){B(e,t,s),n=!0},p(t,s){const o={};s[0]&1632&&(o.paint={"line-width":5,"line-color":t[14](t[5],t[9],t[10]),"line-opacity":t[15](t[5],t[6])}),s[1]&1&&(o.$$scope={dirty:s,ctx:t}),e.$set(o)},i(t){n||(C(e.$$.fragment,t),n=!0)},o(t){F(e.$$.fragment,t),n=!1},d(t){q(e,t)}}}function Ye(l){let e,n;return e=new Ie({props:{data:l[4],$$slots:{default:[Ve]},$$scope:{ctx:l}}}),{c(){j(e.$$.fragment)},m(t,s){B(e,t,s),n=!0},p(t,s){const o={};s[0]&16&&(o.data=t[4]),s[0]&1632|s[1]&1&&(o.$$scope={dirty:s,ctx:t}),e.$set(o)},i(t){n||(C(e.$$.fragment,t),n=!0)},o(t){F(e.$$.fragment,t),n=!1},d(t){q(e,t)}}}function xe(l){let e,n,t,s;function o(i){l[18](i)}let f={style:"https://api.maptiler.com/maps/dataviz/style.json?key=MZEJTanw3WpxRvt7qDfo",standardControls:!0,hash:!0,$$slots:{default:[Ye]},$$scope:{ctx:l}};return l[2]!==void 0&&(f.map=l[2]),n=new Ce({props:f}),Y.push(()=>le(n,"map",o)),{c(){e=$("div"),j(n.$$.fragment),Q(e,"slot","main"),J(e,"position","relative"),J(e,"width","100%"),J(e,"height","100vh")},m(i,r){k(i,e,r),B(n,e,null),s=!0},p(i,r){const m={};r[0]&1648|r[1]&1&&(m.$$scope={dirty:r,ctx:i}),!t&&r[0]&4&&(t=!0,m.map=i[2],se(()=>t=!1)),n.$set(m)},i(i){s||(C(n.$$.fragment,i),s=!0)},o(i){F(n.$$.fragment,i),s=!1},d(i){i&&y(e),q(n)}}}function et(l){let e,n,t,s;return e=new ye({props:{$$slots:{main:[xe],left:[Ze]},$$scope:{ctx:l}}}),t=new Je({props:{loading:l[7]}}),{c(){j(e.$$.fragment),n=N(),j(t.$$.fragment)},m(o,f){B(e,o,f),k(o,n,f),B(t,o,f),s=!0},p(o,f){const i={};f[0]&8063|f[1]&1&&(i.$$scope={dirty:f,ctx:o}),e.$set(i);const r={};f[0]&128&&(r.loading=o[7]),t.$set(r)},i(o){s||(C(e.$$.fragment,o),C(t.$$.fragment,o),s=!0)},o(o){F(e.$$.fragment,o),F(t.$$.fragment,o),s=!1},d(o){o&&y(n),q(e,o),q(t,o)}}}function tt(l){let e=l.properties.way;window.open(`http://openstreetmap.org/way/${e}`,"_blank")}function V(l,e,n){if(l=="lts")return null;if(l=="cost")return de(0,e);if(l=="nearby_amenities")return de(0,n)}function de(l,e){let n=[],t=(e-l)/5;for(let s=0;s<6;s++)n.push(l+s*t);return n}function nt(l,e,n){ve(async()=>{await He(),await Fe()});let t,s,o="",f={type:"FeatureCollection",features:[]},i="Distance",r="cost",m=!1,a=!1,d=[0,0,0,0,0],v=1,h=1,E;async function T(p){n(0,o=""),n(7,a=!0),M(await E.files[0].arrayBuffer())}function M(p){try{n(3,s=new De(new Uint8Array(p))),n(1,i="Distance");let S=s.getBounds();t.fitBounds([[S[0],S[1]],[S[2],S[3]]],{padding:20,animate:!1}),H()}catch(S){window.alert(`Problem importing osm.pbf file: ${S}`)}n(7,a=!1)}async function D(p){if(p!=""){n(7,a=!0);let S=await fetch(`https://assets.od2net.org/pbf_clips/${p}.osm.pbf`);M(await S.arrayBuffer())}}function H(){n(4,f=JSON.parse(s.debugNetwork()));let p=0,S=[0,0,0,0,0];n(9,v=0),n(10,h=0);for(let L of f.features)n(10,h=Math.max(h,L.properties.nearby_amenities)),L.properties.length&&(n(9,v=Math.max(v,L.properties.cost/L.properties.length)),p+=L.properties.length,S[L.properties.lts]+=L.properties.length);n(8,d=S.map(L=>L/p*100))}function R(p,S,L){if(p=="lts")return Le;if(p=="cost")return["case",["==",0,["get","lts"]],A.lts_not_allowed,ae(["/",["get","cost"],["get","length"]],V(p,S,L),ne)];if(p=="nearby_amenities")return ae(["get","nearby_amenities"],V(p,S,L),ne)}function G(p,S){let L=je(1,.5);return p=="nearby_amenities"?["case",["==",0,["get","nearby_amenities"]],0,L]:S?L:["case",["==",0,["get","lts"]],0,L]}let c="";function g(p){n(12,c="Parsing XML"),M(new TextEncoder().encode(p.detail)),n(12,c="")}function _(p){s&&(s.updateCostFunction(p),H())}const O=p=>tt(p.detail.features[0]);function X(p){t=p,n(2,t)}function z(p){Y[p?"unshift":"push"](()=>{E=p,n(11,E)})}function P(p){o=p,n(0,o)}const U=p=>n(12,c=p.detail),x=p=>n(12,c=p.detail);function Z(){r=Me(this),n(5,r)}function u(){m=this.checked,n(6,m)}function w(p){i=p,n(1,i)}return l.$$.update=()=>{l.$$.dirty[0]&1&&D(o),l.$$.dirty[0]&2&&_(i)},[o,i,t,s,f,r,m,a,d,v,h,E,c,T,R,G,g,O,X,z,P,U,x,Z,u,w]}class lt extends ge{constructor(e){super(),$e(this,e,nt,et,be,{},null,[-1,-1])}}new lt({target:document.getElementById("app")});