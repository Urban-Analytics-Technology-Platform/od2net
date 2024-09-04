import{S as ce,i as _e,s as me,ai as se,e as g,h as X,l as y,n as _,E as le,v as k,aj as $e,g as R,f as S,ab as oe,L as be,c as F,a as O,t as w,b as v,d as j,o as he,H as we,x as Q,y as te,q as x,A as ne,r as de,u as ge,M as ye,z as I,_ as ke,a2 as ve,a6 as q,a7 as re,C as Ce,j as Y,k as Le,p as ae,w as Se,ak as ee,a3 as Fe,ah as Oe,af as je,ag as D,a5 as Ee,aa as Me}from"./Layout-63eb556b.js";import{L as Ne,C as Be,O as Pe,_ as qe,J as Te,a as Ae,G as Je,P as He}from"./index-1a302ef2.js";function ie(o,e,s){const t=o.slice();return t[1]=e[s][0],t[2]=e[s][1],t}function fe(o){let e,s,t=o[1]+"",l,n,i,a=o[2]+"",p,b;return{c(){e=g("tr"),s=g("td"),l=R(t),n=S(),i=g("td"),p=R(a),b=S()},m(c,m){y(c,e,m),_(e,s),_(s,l),_(e,n),_(e,i),_(i,p),_(e,b)},p(c,m){m&1&&t!==(t=c[1]+"")&&oe(l,t),m&1&&a!==(a=c[2]+"")&&oe(p,a)},d(c){c&&k(e)}}}function De(o){let e,s,t=se(Object.entries(o[0])),l=[];for(let n=0;n<t.length;n+=1)l[n]=fe(ie(o,t,n));return{c(){e=g("table"),s=g("tbody");for(let n=0;n<l.length;n+=1)l[n].c();X(e,"class","svelte-lh2o9l")},m(n,i){y(n,e,i),_(e,s);for(let a=0;a<l.length;a+=1)l[a]&&l[a].m(s,null)},p(n,[i]){if(i&1){t=se(Object.entries(n[0]));let a;for(a=0;a<t.length;a+=1){const p=ie(n,t,a);l[a]?l[a].p(p,i):(l[a]=fe(p),l[a].c(),l[a].m(s,null))}for(;a<l.length;a+=1)l[a].d(1);l.length=t.length}},i:le,o:le,d(n){n&&k(e),$e(l,n)}}}function Ge(o,e,s){let{properties:t}=e;return o.$$set=l=>{"properties"in l&&s(0,t=l.properties)},[t]}class Ie extends ce{constructor(e){super(),_e(this,e,Ge,De,me,{properties:0})}}function pe(o){let e,s,t,l,n,i,a,p,b,c,m,h,$,E,M,C,G,A,J,d,u,N,T,H,z;const U=[Xe,Re],B=[];function W(r,f){return r[5]=="lts"?0:1}m=W(o),h=B[m]=U[m](o);function V(r){o[23](r)}let Z={};return o[1]!==void 0&&(Z.cost=o[1]),u=new Ae({props:Z}),Q.push(()=>te(u,"cost",V)),{c(){e=g("hr"),s=S(),t=g("div"),l=g("label"),n=R(`Color edges by:
          `),i=g("select"),a=g("option"),a.textContent="LTS",p=g("option"),p.textContent="Edge cost (relative to length)",b=g("option"),b.textContent="Nearby amenities",c=S(),h.c(),$=S(),E=g("div"),M=g("label"),C=g("input"),G=R(`
          Show cyclists not allowed`),A=S(),J=g("hr"),d=S(),F(u.$$.fragment),a.__value="lts",Y(a,a.__value),p.__value="cost",Y(p,p.__value),b.__value="nearby_amenities",Y(b,b.__value),o[5]===void 0&&Le(()=>o[21].call(i)),X(C,"type","checkbox"),I(M,"color",q.lts_not_allowed)},m(r,f){y(r,e,f),y(r,s,f),y(r,t,f),_(t,l),_(l,n),_(l,i),_(i,a),_(i,p),_(i,b),ae(i,o[5],!0),y(r,c,f),B[m].m(r,f),y(r,$,f),y(r,E,f),_(E,M),_(M,C),C.checked=o[6],_(M,G),y(r,A,f),y(r,J,f),y(r,d,f),O(u,r,f),T=!0,H||(z=[x(i,"change",o[21]),x(C,"change",o[22])],H=!0)},p(r,f){f&32&&ae(i,r[5]);let L=m;m=W(r),m===L?B[m].p(r,f):(de(),v(B[L],1,1,()=>{B[L]=null}),ge(),h=B[m],h?h.p(r,f):(h=B[m]=U[m](r),h.c()),w(h,1),h.m($.parentNode,$)),f&64&&(C.checked=r[6]);const P={};!N&&f&2&&(N=!0,P.cost=r[1],ne(()=>N=!1)),u.$set(P)},i(r){T||(w(h),w(u.$$.fragment,r),T=!0)},o(r){v(h),v(u.$$.fragment,r),T=!1},d(r){r&&(k(e),k(s),k(t),k(c),k($),k(E),k(A),k(J),k(d)),B[m].d(r),j(u,r),H=!1,Se(z)}}}function Re(o){let e,s;return e=new Oe({props:{colorScale:ee,limits:K(o[5],o[9],o[10]),decimalPlaces:1}}),{c(){F(e.$$.fragment)},m(t,l){O(e,t,l),s=!0},p(t,l){const n={};l&1568&&(n.limits=K(t[5],t[9],t[10])),e.$set(n)},i(t){s||(w(e.$$.fragment,t),s=!0)},o(t){v(e.$$.fragment,t),s=!1},d(t){j(e,t)}}}function Xe(o){let e,s,t,l;return e=new je({props:{rows:[[`${D.lts1}: ${o[8][1].toFixed(0)}% of roads by distance`,q.lts1],[`${D.lts2}: ${o[8][2].toFixed(0)}%`,q.lts2],[`${D.lts3}: ${o[8][3].toFixed(0)}%`,q.lts3],[`${D.lts4}: ${o[8][4].toFixed(0)}%`,q.lts4]]}}),{c(){F(e.$$.fragment),s=S(),t=g("p"),t.innerHTML='Note: LTS model from <a href="https://github.com/BikeOttawa/stressmodel/blob/master/stressmodel.js" target="_blank">BikeOttawa</a>'},m(n,i){O(e,n,i),y(n,s,i),y(n,t,i),l=!0},p(n,i){const a={};i&256&&(a.rows=[[`${D.lts1}: ${n[8][1].toFixed(0)}% of roads by distance`,q.lts1],[`${D.lts2}: ${n[8][2].toFixed(0)}%`,q.lts2],[`${D.lts3}: ${n[8][3].toFixed(0)}%`,q.lts3],[`${D.lts4}: ${n[8][4].toFixed(0)}%`,q.lts4]]),e.$set(a)},i(n){l||(w(e.$$.fragment,n),l=!0)},o(n){v(e.$$.fragment,n),l=!1},d(n){n&&(k(s),k(t)),j(e,n)}}}function ze(o){let e,s,t,l,n,i,a,p,b,c,m,h,$,E,M,C,G;s=new we({props:{app:"costs"}});function A(u){o[18](u)}let J={};o[0]!==void 0&&(J.example=o[0]),c=new Be({props:J}),Q.push(()=>te(c,"example",A)),$=new Pe({props:{map:o[2]}}),$.$on("gotXml",o[15]),$.$on("loading",o[19]),$.$on("error",o[20]);let d=o[3]&&pe(o);return{c(){e=g("div"),F(s.$$.fragment),t=S(),l=g("label"),n=R("Open an "),i=g("i"),i.textContent=".osm.pbf",a=R(`
      file
      `),p=g("input"),b=S(),F(c.$$.fragment),h=S(),F($.$$.fragment),E=S(),d&&d.c(),X(p,"type","file"),X(e,"slot","left")},m(u,N){y(u,e,N),O(s,e,null),_(e,t),_(e,l),_(l,n),_(l,i),_(l,a),_(l,p),o[17](p),_(e,b),O(c,e,null),_(e,h),O($,e,null),_(e,E),d&&d.m(e,null),M=!0,C||(G=x(p,"change",o[12]),C=!0)},p(u,N){const T={};!m&&N&1&&(m=!0,T.example=u[0],ne(()=>m=!1)),c.$set(T);const H={};N&4&&(H.map=u[2]),$.$set(H),u[3]?d?(d.p(u,N),N&8&&w(d,1)):(d=pe(u),d.c(),w(d,1),d.m(e,null)):d&&(de(),v(d,1,1,()=>{d=null}),ge())},i(u){M||(w(s.$$.fragment,u),w(c.$$.fragment,u),w($.$$.fragment,u),w(d),M=!0)},o(u){v(s.$$.fragment,u),v(c.$$.fragment,u),v($.$$.fragment,u),v(d),M=!1},d(u){u&&k(e),j(s),o[17](null),j(c),j($),d&&d.d(),C=!1,G()}}}function Ue(o){let e,s,t;return s=new Ie({props:{properties:o[28]}}),{c(){e=g("div"),F(s.$$.fragment),I(e,"max-width","30vw"),I(e,"max-height","60vh"),I(e,"overflow","auto")},m(l,n){y(l,e,n),O(s,e,null),t=!0},p(l,n){const i={};n&268435456&&(i.properties=l[28]),s.$set(i)},i(l){t||(w(s.$$.fragment,l),t=!0)},o(l){v(s.$$.fragment,l),t=!1},d(l){l&&k(e),j(s)}}}function We(o){let e,s;return e=new Me({props:{$$slots:{default:[Ue,({props:t})=>({28:t}),({props:t})=>t?268435456:0]},$$scope:{ctx:o}}}),{c(){F(e.$$.fragment)},m(t,l){O(e,t,l),s=!0},p(t,l){const n={};l&805306368&&(n.$$scope={dirty:l,ctx:t}),e.$set(n)},i(t){s||(w(e.$$.fragment,t),s=!0)},o(t){v(e.$$.fragment,t),s=!1},d(t){j(e,t)}}}function Ze(o){let e,s;return e=new Ee({props:{manageHoverState:!0,hoverCursor:"pointer",paint:{"line-width":5,"line-color":o[13](o[5],o[9],o[10]),"line-opacity":o[14](o[5],o[6])},beforeId:"Road labels",$$slots:{default:[We]},$$scope:{ctx:o}}}),e.$on("click",Ye),{c(){F(e.$$.fragment)},m(t,l){O(e,t,l),s=!0},p(t,l){const n={};l&1632&&(n.paint={"line-width":5,"line-color":t[13](t[5],t[9],t[10]),"line-opacity":t[14](t[5],t[6])}),l&536870912&&(n.$$scope={dirty:l,ctx:t}),e.$set(n)},i(t){s||(w(e.$$.fragment,t),s=!0)},o(t){v(e.$$.fragment,t),s=!1},d(t){j(e,t)}}}function Ke(o){let e,s,t,l;return e=new Je({props:{data:o[4],$$slots:{default:[Ze]},$$scope:{ctx:o}}}),t=new He({}),{c(){F(e.$$.fragment),s=S(),F(t.$$.fragment)},m(n,i){O(e,n,i),y(n,s,i),O(t,n,i),l=!0},p(n,i){const a={};i&16&&(a.data=n[4]),i&536872544&&(a.$$scope={dirty:i,ctx:n}),e.$set(a)},i(n){l||(w(e.$$.fragment,n),w(t.$$.fragment,n),l=!0)},o(n){v(e.$$.fragment,n),v(t.$$.fragment,n),l=!1},d(n){n&&k(s),j(e,n),j(t,n)}}}function Qe(o){let e,s,t,l;function n(a){o[16](a)}let i={style:"https://api.maptiler.com/maps/dataviz/style.json?key=MZEJTanw3WpxRvt7qDfo",standardControls:!0,hash:!0,$$slots:{default:[Ke]},$$scope:{ctx:o}};return o[2]!==void 0&&(i.map=o[2]),s=new ye({props:i}),Q.push(()=>te(s,"map",n)),{c(){e=g("div"),F(s.$$.fragment),X(e,"slot","main"),I(e,"position","relative"),I(e,"width","100%"),I(e,"height","100vh")},m(a,p){y(a,e,p),O(s,e,null),l=!0},p(a,p){const b={};p&536872560&&(b.$$scope={dirty:p,ctx:a}),!t&&p&4&&(t=!0,b.map=a[2],ne(()=>t=!1)),s.$set(b)},i(a){l||(w(s.$$.fragment,a),l=!0)},o(a){v(s.$$.fragment,a),l=!1},d(a){a&&k(e),j(s)}}}function Ve(o){let e,s,t,l;return e=new be({props:{$$slots:{main:[Qe],left:[ze]},$$scope:{ctx:o}}}),t=new Ne({props:{loading:o[7]}}),{c(){F(e.$$.fragment),s=S(),F(t.$$.fragment)},m(n,i){O(e,n,i),y(n,s,i),O(t,n,i),l=!0},p(n,[i]){const a={};i&536875007&&(a.$$scope={dirty:i,ctx:n}),e.$set(a);const p={};i&128&&(p.loading=n[7]),t.$set(p)},i(n){l||(w(e.$$.fragment,n),w(t.$$.fragment,n),l=!0)},o(n){v(e.$$.fragment,n),v(t.$$.fragment,n),l=!1},d(n){n&&k(s),j(e,n),j(t,n)}}}function Ye(o){let e=o.detail.features[0].properties.way;window.open(`http://openstreetmap.org/way/${e}`,"_blank")}function K(o,e,s){if(o=="lts")return[];if(o=="cost")return ue(0,e);if(o=="nearby_amenities")return ue(0,s);throw new Error("unreachable")}function ue(o,e){let s=[],t=(e-o)/5;for(let l=0;l<6;l++)s.push(o+l*t);return s}function xe(o,e,s){he(async()=>{await qe(),await ke()});let t,l,n="",i={type:"FeatureCollection",features:[]},a="Distance",p="cost",b=!1,c="",m=[0,0,0,0,0],h=1,$=1,E;async function M(r){s(0,n=""),s(7,c="Loading file"),C(await E.files[0].arrayBuffer())}function C(r){try{s(3,l=new Te(new Uint8Array(r))),s(1,a="Distance");let f=l.getBounds();t.fitBounds([[f[0],f[1]],[f[2],f[3]]],{padding:20,animate:!1}),A()}catch(f){window.alert(`Problem importing osm.pbf file: ${f}`)}s(7,c="")}async function G(r){if(r!=""){s(7,c=`Loading ${r}`);let f=await fetch(`https://assets.od2net.org/pbf_clips/${r}.osm.pbf`);C(await f.arrayBuffer())}}function A(){s(4,i=JSON.parse(l.debugNetwork()));let r=0,f=[0,0,0,0,0];s(9,h=0),s(10,$=0);for(let L of i.features){let P=L.properties;s(10,$=Math.max($,P.nearby_amenities)),P.length&&(s(9,h=Math.max(h,P.forward_cost/P.length)),r+=P.length,f[P.lts]+=P.length)}s(8,m=f.map(L=>L/r*100))}function J(r,f,L){if(r=="lts")return ve;if(r=="cost")return["case",["==",0,["get","lts"]],q.lts_not_allowed,re(["/",["get","forward_cost"],["get","length"]],K(r,f,L),ee)];if(r=="nearby_amenities")return re(["get","nearby_amenities"],K(r,f,L),ee);throw new Error("unreachable")}function d(r,f){let L=Fe(1,.5);return r=="nearby_amenities"?["case",["==",0,["get","nearby_amenities"]],0,L]:f?L:["case",["==",0,["get","lts"]],0,L]}function u(r){s(7,c="Parsing XML"),C(new TextEncoder().encode(r.detail)),s(7,c="")}function N(r){l&&(l.updateCostFunction(r),A())}function T(r){t=r,s(2,t)}function H(r){Q[r?"unshift":"push"](()=>{E=r,s(11,E)})}function z(r){n=r,s(0,n)}const U=r=>s(7,c=r.detail),B=r=>s(7,c=r.detail);function W(){p=Ce(this),s(5,p)}function V(){b=this.checked,s(6,b)}function Z(r){a=r,s(1,a)}return o.$$.update=()=>{o.$$.dirty&1&&G(n),o.$$.dirty&2&&N(a)},[n,a,t,l,i,p,b,c,m,h,$,E,M,J,d,u,T,H,z,U,B,W,V,Z]}class et extends ce{constructor(e){super(),_e(this,e,xe,Ve,me,{})}}new et({target:document.getElementById("app")});
