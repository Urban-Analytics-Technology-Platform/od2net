import{S as K,i as U,s as Z,G as ge,e as y,h as M,z as R,l as k,H as Y,q as S,I as be,J as he,K as ye,N as we,t as v,b as z,v as L,w as ve,O as ke,Q as Le,R as W,T as $,m as Ce,g as j,f as B,D as re,j as H,k as Me,n as b,p as x,E as P,U as ee,V as ze,W as Oe,X as F,C as Se,Y as Te,L as Be,c as E,a as I,d as A,o as De,r as te,u as ne,M as He,x as G,y as Q,A as V,_ as Ee,Z as Ie}from"./Layout-09a07ccd.js";import{_ as Ae,J as Ne,G as qe}from"./@vite-plugin-wasm-pack@wasm-od2net-379c176f.js";import{g as Je,S as Re,L as je}from"./SidebarControls-50627875.js";function Pe(l){let e,t,a,n,o,i,r;const s=l[16].default,u=ge(s,l,l[15],null);return{c(){e=y("div"),u&&u.c(),M(e,"tabindex",t=l[1]?0:void 0),M(e,"role",a=l[1]?"button":void 0),R(e,"z-index",l[2])},m(f,p){k(f,e,p),u&&u.m(e,null),o=!0,i||(r=[Y(l[6].call(null,e)),Y(n=Fe.call(null,e,l[0])),S(e,"click",l[17]),S(e,"dblclick",l[18]),S(e,"contextmenu",l[19]),S(e,"mouseenter",l[20]),S(e,"mouseleave",l[21]),S(e,"mousemove",l[22]),S(e,"keydown",l[7])],i=!0)},p(f,[p]){u&&u.p&&(!o||p&32768)&&be(u,s,f,f[15],o?ye(s,f[15],p,null):he(f[15]),null),(!o||p&2&&t!==(t=f[1]?0:void 0))&&M(e,"tabindex",t),(!o||p&2&&a!==(a=f[1]?"button":void 0))&&M(e,"role",a),n&&we(n.update)&&p&1&&n.update.call(null,f[0]),p&4&&R(e,"z-index",f[2])},i(f){o||(v(u,f),o=!0)},o(f){z(u,f),o=!1},d(f){f&&L(e),u&&u.d(f),i=!1,ve(r)}}}function Fe(l,e){const t=l.className;function a(n){n?l.className=`${t} ${n}`:l.className=t}return a(e),{update:a}}function Ge(l,e,t){let a,n,o,{$$slots:i={},$$scope:r}=e,{lngLat:s}=e,{class:u=void 0}=e,{interactive:f=!0}=e,{asButton:p=!1}=e,{draggable:d=!1}=e,{feature:g=null}=e,{offset:m=void 0}=e,{zIndex:_=void 0}=e;const h=ke(),{map:w,layerEvent:C,self:D}=Le();W(l,w,c=>t(24,o=c)),W(l,C,c=>t(23,a=c)),W(l,D,c=>t(14,n=c));function T(c){$(D,n=new Ce.Marker({element:c,draggable:d,offset:m}).setLngLat(s).addTo(o),n);const N=()=>O("dragstart"),q=()=>{X(),O("drag")},J=()=>{X(),O("dragend")};return d&&(n.on("dragstart",N),n.on("drag",q),n.on("dragend",J)),{destroy(){d&&(n==null||n.off("dragstart",N),n==null||n.off("drag",q),n==null||n.off("dragend",J)),n==null||n.remove()}}}function X(){let c=n==null?void 0:n.getLngLat();c&&(Array.isArray(s)?t(9,s=[c.lng,c.lat]):s&&"lon"in s?t(9,s={lon:c.lng,lat:c.lat}):t(9,s=c))}function ue(c){c.key===" "&&(c.preventDefault(),c.stopPropagation(),O("click"))}function O(c){if(!f)return;let N=n==null?void 0:n.getLngLat();if(!N)return;const q=[N.lng,N.lat];let J={map:o,marker:n,lngLat:q,features:[{type:"Feature",properties:(g==null?void 0:g.properties)??{},geometry:{type:"Point",coordinates:q}}]};$(C,a={...J,layerType:"marker",type:c},a),h(c,J)}const fe=()=>O("click"),ce=()=>O("dblclick"),_e=()=>O("contextmenu"),de=c=>{O("mouseenter")},me=()=>{O("mouseleave")},pe=()=>O("mousemove");return l.$$set=c=>{"lngLat"in c&&t(9,s=c.lngLat),"class"in c&&t(0,u=c.class),"interactive"in c&&t(10,f=c.interactive),"asButton"in c&&t(1,p=c.asButton),"draggable"in c&&t(11,d=c.draggable),"feature"in c&&t(12,g=c.feature),"offset"in c&&t(13,m=c.offset),"zIndex"in c&&t(2,_=c.zIndex),"$$scope"in c&&t(15,r=c.$$scope)},l.$$.update=()=>{l.$$.dirty&16896&&(n==null||n.setLngLat(s)),l.$$.dirty&24576&&(n==null||n.setOffset(m??[0,0]))},[u,p,_,w,C,D,T,ue,O,s,f,d,g,m,n,r,i,fe,ce,_e,de,me,pe]}class We extends K{constructor(e){super(),U(this,e,Ge,Pe,Z,{lngLat:9,class:0,interactive:10,asButton:1,draggable:11,feature:12,offset:13,zIndex:2})}}const Ke='<svg xmlns="http://www.w3.org/2000/svg" height="24px" viewBox="0 0 24 24" width="24px" fill="#000000"><path d="M0 0h24v24H0z" fill="none"/><path d="M12 2C8.13 2 5 5.13 5 9c0 5.25 7 13 7 13s7-7.75 7-13c0-3.87-3.13-7-7-7zm0 9.5c-1.38 0-2.5-1.12-2.5-2.5s1.12-2.5 2.5-2.5 2.5 1.12 2.5 2.5-1.12 2.5-2.5 2.5z"/></svg>',{Map:Ue}=Je;function le(l,e,t){const a=l.slice();return a[6]=e[t][0],a[7]=e[t][1],a[8]=e,a[9]=t,a}function ae(l){let e,t=[],a=new Ue,n=ee(l[1].entries());const o=i=>i[6];for(let i=0;i<n.length;i+=1){let r=le(l,n,i),s=o(r);a.set(s,t[i]=se(s,r))}return{c(){e=y("ul");for(let i=0;i<t.length;i+=1)t[i].c()},m(i,r){k(i,e,r);for(let s=0;s<t.length;s+=1)t[s]&&t[s].m(e,null)},p(i,r){r&2&&(n=ee(i[1].entries()),t=ze(t,r,o,1,i,n,a,e,Te,se,null,le))},d(i){i&&L(e);for(let r=0;r<t.length;r+=1)t[r].d()}}}function se(l,e){let t,a,n=e[6]+"",o,i,r,s,u;function f(){e[4].call(i,e[8],e[9])}return{key:l,first:null,c(){t=y("li"),a=y("label"),o=j(n),i=y("input"),r=B(),M(i,"type","number"),M(i,"min","1.0"),M(i,"step","0.1"),this.first=t},m(p,d){k(p,t,d),b(t,a),b(a,o),b(a,i),H(i,e[7]),b(t,r),s||(u=S(i,"input",f),s=!0)},p(p,d){e=p,d&2&&n!==(n=e[6]+"")&&Oe(o,n),d&2&&F(i.value)!==e[7]&&H(i,e[7])},d(p){p&&L(t),s=!1,u()}}}function Ze(l){let e,t,a,n,o,i,r,s,u,f,p,d=l[0]=="OsmHighwayType"&&ae(l);return{c(){e=y("div"),t=y("label"),a=j(`Cost function:
    `),n=y("select"),o=y("option"),o.textContent="Distance",i=y("option"),i.textContent="Avoid main roads",r=y("option"),r.textContent="Set a weight per OSM highway type",s=B(),d&&d.c(),u=re(),o.__value="Distance",H(o,o.__value),i.__value="AvoidMainRoads",H(i,i.__value),r.__value="OsmHighwayType",H(r,r.__value),l[0]===void 0&&Me(()=>l[3].call(n))},m(g,m){k(g,e,m),b(e,t),b(t,a),b(t,n),b(n,o),b(n,i),b(n,r),x(n,l[0],!0),k(g,s,m),d&&d.m(g,m),k(g,u,m),f||(p=S(n,"change",l[3]),f=!0)},p(g,[m]){m&1&&x(n,g[0]),g[0]=="OsmHighwayType"?d?d.p(g,m):(d=ae(g),d.c(),d.m(u.parentNode,u)):d&&(d.d(1),d=null)},i:P,o:P,d(g){g&&(L(e),L(s),L(u)),d&&d.d(g),f=!1,p()}}}function Qe(l,e,t){let{cost:a}=e,n="Distance",o=new Map(["cycleway","footway","living_street","motorway","motorway_link","path","pedestrian","primary","primary_link","residential","secondary","secondary_link","service","steps","tertiary","tertiary_link","track","trunk","trunk_link","unclassified"].map(u=>[u,1]));function i(u){u=="OsmHighwayType"?t(2,a={OsmHighwayType:o}):t(2,a=u)}function r(){n=Se(this),t(0,n)}function s(u,f){u[f][1]=F(this.value),t(1,o)}return l.$$set=u=>{"cost"in u&&t(2,a=u.cost)},l.$$.update=()=>{l.$$.dirty&1&&i(n)},[n,o,a,r,s]}class Ve extends K{constructor(e){super(),U(this,e,Qe,Ze,Z,{cost:2})}}function ie(l){let e,t,a,n,o,i,r,s,u,f,p,d;function g(_){l[14](_)}let m={};return l[1]!==void 0&&(m.cost=l[1]),s=new Ve({props:m}),G.push(()=>Q(s,"cost",g)),{c(){e=y("div"),t=y("label"),a=j("Max requests (limit for faster updates):"),n=y("br"),o=B(),i=y("input"),r=B(),E(s.$$.fragment),M(i,"type","number"),M(i,"min",1)},m(_,h){k(_,e,h),b(e,t),b(t,a),b(t,n),b(t,o),b(t,i),H(i,l[0]),k(_,r,h),I(s,_,h),f=!0,p||(d=S(i,"input",l[13]),p=!0)},p(_,h){h&1&&F(i.value)!==_[0]&&H(i,_[0]);const w={};!u&&h&2&&(u=!0,w.cost=_[1],V(()=>u=!1)),s.$set(w)},i(_){f||(v(s.$$.fragment,_),f=!0)},o(_){z(s.$$.fragment,_),f=!1},d(_){_&&(L(e),L(r)),A(s,_),p=!1,d()}}}function oe(l){let e,t,a,n;return a=new Re({props:{outputMetadata:l[5].metadata,map:l[2],controls:l[7]}}),{c(){e=y("hr"),t=B(),E(a.$$.fragment)},m(o,i){k(o,e,i),k(o,t,i),I(a,o,i),n=!0},p(o,i){const r={};i&32&&(r.outputMetadata=o[5].metadata),i&4&&(r.map=o[2]),a.$set(r)},i(o){n||(v(a.$$.fragment,o),n=!0)},o(o){z(a.$$.fragment,o),n=!1},d(o){o&&(L(e),L(t)),A(a,o)}}}function Xe(l){let e,t,a,n,o,i,r,s,u,f,p,d,g,m=l[3]&&ie(l),_=l[5].metadata&&oe(l);return{c(){e=y("div"),t=y("h1"),t.textContent="od2net interactive mode",a=B(),n=y("label"),o=j("Open a "),i=y("i"),i.textContent=".bin",r=j(` network file
      `),s=y("input"),u=B(),m&&m.c(),f=B(),_&&_.c(),M(s,"type","file"),M(e,"slot","left")},m(h,w){k(h,e,w),b(e,t),b(e,a),b(e,n),b(n,o),b(n,i),b(n,r),b(n,s),l[12](s),b(e,u),m&&m.m(e,null),b(e,f),_&&_.m(e,null),p=!0,d||(g=S(s,"change",l[8]),d=!0)},p(h,w){h[3]?m?(m.p(h,w),w&8&&v(m,1)):(m=ie(h),m.c(),v(m,1),m.m(e,f)):m&&(te(),z(m,1,1,()=>{m=null}),ne()),h[5].metadata?_?(_.p(h,w),w&32&&v(_,1)):(_=oe(h),_.c(),v(_,1),_.m(e,null)):_&&(te(),z(_,1,1,()=>{_=null}),ne())},i(h){p||(v(m),v(_),p=!0)},o(h){z(m),z(_),p=!1},d(h){h&&L(e),l[12](null),m&&m.d(),_&&_.d(),d=!1,g()}}}function Ye(l){let e,t;return{c(){e=new Ie(!1),t=re(),e.a=t},m(a,n){e.m(Ke,a,n),k(a,t,n)},p:P,d(a){a&&(L(t),e.d())}}}function $e(l){let e,t;return e=new je({props:{controls:l[7]}}),{c(){E(e.$$.fragment)},m(a,n){I(e,a,n),t=!0},p:P,i(a){t||(v(e.$$.fragment,a),t=!0)},o(a){z(e.$$.fragment,a),t=!1},d(a){A(e,a)}}}function xe(l){let e,t,a,n,o;function i(s){l[10](s)}let r={draggable:!0,$$slots:{default:[Ye]},$$scope:{ctx:l}};return l[4]!==void 0&&(r.lngLat=l[4]),e=new We({props:r}),G.push(()=>Q(e,"lngLat",i)),e.$on("dragend",l[9]),n=new qe({props:{data:l[5],$$slots:{default:[$e]},$$scope:{ctx:l}}}),{c(){E(e.$$.fragment),a=B(),E(n.$$.fragment)},m(s,u){I(e,s,u),k(s,a,u),I(n,s,u),o=!0},p(s,u){const f={};u&32768&&(f.$$scope={dirty:u,ctx:s}),!t&&u&16&&(t=!0,f.lngLat=s[4],V(()=>t=!1)),e.$set(f);const p={};u&32&&(p.data=s[5]),u&32768&&(p.$$scope={dirty:u,ctx:s}),n.$set(p)},i(s){o||(v(e.$$.fragment,s),v(n.$$.fragment,s),o=!0)},o(s){z(e.$$.fragment,s),z(n.$$.fragment,s),o=!1},d(s){s&&L(a),A(e,s),A(n,s)}}}function et(l){let e,t,a,n;function o(r){l[11](r)}let i={style:"https://api.maptiler.com/maps/dataviz/style.json?key=MZEJTanw3WpxRvt7qDfo",standardControls:!0,hash:!0,$$slots:{default:[xe]},$$scope:{ctx:l}};return l[2]!==void 0&&(i.map=l[2]),t=new He({props:i}),G.push(()=>Q(t,"map",o)),{c(){e=y("div"),E(t.$$.fragment),M(e,"slot","main"),R(e,"position","relative"),R(e,"width","100%"),R(e,"height","100vh")},m(r,s){k(r,e,s),I(t,e,null),n=!0},p(r,s){const u={};s&32816&&(u.$$scope={dirty:s,ctx:r}),!a&&s&4&&(a=!0,u.map=r[2],V(()=>a=!1)),t.$set(u)},i(r){n||(v(t.$$.fragment,r),n=!0)},o(r){z(t.$$.fragment,r),n=!1},d(r){r&&L(e),A(t)}}}function tt(l){let e,t;return e=new Be({props:{$$slots:{main:[et],left:[Xe]},$$scope:{ctx:l}}}),{c(){E(e.$$.fragment)},m(a,n){I(e,a,n),t=!0},p(a,[n]){const o={};n&32895&&(o.$$scope={dirty:n,ctx:a}),e.$set(o)},i(a){t||(v(e.$$.fragment,a),t=!0)},o(a){z(e.$$.fragment,a),t=!1},d(a){A(e,a)}}}function nt(l,e,t){De(async()=>{await Ae(),await Ee()});let a,n,o={lng:0,lat:0},i={type:"FeatureCollection",features:[]},r=1e3,s="Distance",u={maxCount:1e3,originRadius:3,destinationRadius:3,streetviewOn:!1},f;async function p(C){try{let D=await f.files[0].arrayBuffer();t(3,n=new Ne(new Uint8Array(D)));let T=n.getBounds();a.fitBounds([[T[0],T[1]],[T[2],T[3]]],{padding:20,animate:!1}),t(4,o.lng=(T[0]+T[2])/2,o),t(4,o.lat=(T[1]+T[3])/2,o),d()}catch(D){window.alert(`Problem loading network file: ${D}`)}}function d(){n&&t(5,i=JSON.parse(n.recalculate({lng:o.lng,lat:o.lat,max_requests:r,cost:s})))}function g(C){o=C,t(4,o)}function m(C){a=C,t(2,a)}function _(C){G[C?"unshift":"push"](()=>{f=C,t(6,f)})}function h(){r=F(this.value),t(0,r)}function w(C){s=C,t(1,s)}return l.$$.update=()=>{l.$$.dirty&3&&d()},[r,s,a,n,o,i,f,u,p,d,g,m,_,h,w]}class lt extends K{constructor(e){super(),U(this,e,nt,tt,Z,{})}}new lt({target:document.getElementById("app")});