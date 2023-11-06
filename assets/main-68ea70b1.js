import{S as te,i as oe,s as T,m as re,P as ue,L as se,c as q,a as E,t as d,b,d as O,o as pe,H as fe,e as _,f as N,g as H,h as J,j as C,k as me,l as S,n as m,p as W,q as Y,r as R,u as A,v as P,w as ce,M as de,x as j,y as ne,z as D,A as ae,_ as _e,B as Q,F as ge,C as be,D as le,E as ie}from"./Legend-52595ab8.js";import{S as he,L as ve}from"./SidebarControls-958a55ca.js";let F="pmtilesSource";async function ye(a){let e=await a.getHeader(),n=await a.getMetadata();return{bounds:[e.minLon,e.minLat,e.maxLon,e.maxLat],outputMetadata:JSON.parse(n.description),minZoom:e.minZoom,maxZoom:e.maxZoom}}function ke(a,e,n){let{pmtiles:t}=e,{map:o}=e,{outputMetadata:l=null}=e,u=new ue;re.addProtocol("pmtiles",u.tile);async function g(i){r(F),u.add(i);let p=await ye(i);o.addSource(F,{type:"vector",tiles:["pmtiles://"+i.source.getKey()+"/{z}/{x}/{y}"],minzoom:p.minZoom,maxzoom:p.maxZoom,bounds:p.bounds}),o.fitBounds(p.bounds,{padding:100,duration:500}),n(0,l=p.outputMetadata)}function r(i){if(o.getSource(i)){let p=[];for(let s of o.getStyle().layers)"source"in s&&s.source==i&&p.push(s.id);for(let s of p)o.removeLayer(s);o.removeSource(i)}}return a.$$set=i=>{"pmtiles"in i&&n(1,t=i.pmtiles),"map"in i&&n(2,o=i.map),"outputMetadata"in i&&n(0,l=i.outputMetadata)},a.$$.update=()=>{a.$$.dirty&2&&(t?g(t):r(F))},[l,t,o]}class $e extends te{constructor(e){super(),oe(this,e,ke,null,T,{pmtiles:1,map:2,outputMetadata:0})}}function U(a){let e,n,t;return{c(){e=H("Open a "),n=_("i"),n.textContent=".pmtiles",t=H(` file produced by the tool. Note this file stays in
        your browser; it doesn't get uploaded anywhere.`)},m(o,l){S(o,e,l),S(o,n,l),S(o,t,l)},d(o){o&&(P(e),P(n),P(t))}}}function V(a){let e,n;return e=new he({props:{outputMetadata:a[3],map:a[0],controls:a[5]}}),{c(){q(e.$$.fragment)},m(t,o){E(e,t,o),n=!0},p(t,o){const l={};o&8&&(l.outputMetadata=t[3]),o&1&&(l.map=t[0]),e.$set(l)},i(t){n||(d(e.$$.fragment,t),n=!0)},o(t){b(e.$$.fragment,t),n=!1},d(t){O(e,t)}}}function we(a){let e,n,t,o,l,u,g,r,i,p,s,k,$,f,y,w,M,L,Z,G,I,B,K;n=new fe({props:{app:"main"}});let h=a[2]==null&&U(),c=a[3]&&V(a);return{c(){e=_("div"),q(n.$$.fragment),t=N(),o=_("label"),h&&h.c(),l=N(),u=_("input"),g=N(),r=_("div"),i=_("label"),p=H(`Or load an example:
        `),s=_("select"),k=_("option"),k.textContent="Custom file loaded",$=_("option"),$.textContent="Edinburgh",f=_("option"),f.textContent="England (2011 home-to-work)",y=_("option"),y.textContent="Liverpool (direct)",w=_("option"),w.textContent="Liverpool (quiet)",M=_("option"),M.textContent="London",L=_("option"),L.textContent="Seattle",Z=_("option"),Z.textContent="York",G=N(),c&&c.c(),J(u,"type","file"),k.__value="",C(k,k.__value),$.__value="edinburgh",C($,$.__value),f.__value="england_2011_home_to_work",C(f,f.__value),y.__value="liverpool_direct",C(y,y.__value),w.__value="liverpool_quiet",C(w,w.__value),M.__value="london",C(M,M.__value),L.__value="seattle",C(L,L.__value),Z.__value="york",C(Z,Z.__value),a[1]===void 0&&me(()=>a[10].call(s)),J(e,"slot","left")},m(v,z){S(v,e,z),E(n,e,null),m(e,t),m(e,o),h&&h.m(o,null),m(o,l),m(o,u),a[9](u),m(e,g),m(e,r),m(r,i),m(i,p),m(i,s),m(s,k),m(s,$),m(s,f),m(s,y),m(s,w),m(s,M),m(s,L),m(s,Z),W(s,a[1],!0),m(e,G),c&&c.m(e,null),I=!0,B||(K=[Y(u,"change",a[6]),Y(s,"change",a[10])],B=!0)},p(v,z){v[2]==null?h||(h=U(),h.c(),h.m(o,l)):h&&(h.d(1),h=null),z&2&&W(s,v[1]),v[3]?c?(c.p(v,z),z&8&&d(c,1)):(c=V(v),c.c(),d(c,1),c.m(e,null)):c&&(R(),b(c,1,1,()=>{c=null}),A())},i(v){I||(d(n.$$.fragment,v),d(c),I=!0)},o(v){b(n.$$.fragment,v),b(c),I=!1},d(v){v&&P(e),O(n),h&&h.d(),a[9](null),c&&c.d(),B=!1,ce(K)}}}function X(a){let e=a[3],n,t,o=x(a);return{c(){o.c(),n=le()},m(l,u){o.m(l,u),S(l,n,u),t=!0},p(l,u){u&8&&T(e,e=l[3])?(R(),b(o,1,1,ie),A(),o=x(l),o.c(),d(o,1),o.m(n.parentNode,n)):o.p(l,u)},i(l){t||(d(o),t=!0)},o(l){b(o),t=!1},d(l){l&&P(n),o.d(l)}}}function x(a){let e,n;return e=new ve({props:{sourceOverride:{source:"pmtilesSource",sourceLayer:"rnet"},controls:a[5]}}),{c(){q(e.$$.fragment)},m(t,o){E(e,t,o),n=!0},p:ie,i(t){n||(d(e.$$.fragment,t),n=!0)},o(t){b(e.$$.fragment,t),n=!1},d(t){O(e,t)}}}function Me(a){let e,n,t=a[3]&&X(a);return{c(){t&&t.c(),e=le()},m(o,l){t&&t.m(o,l),S(o,e,l),n=!0},p(o,l){o[3]?t?(t.p(o,l),l&8&&d(t,1)):(t=X(o),t.c(),d(t,1),t.m(e.parentNode,e)):t&&(R(),b(t,1,1,()=>{t=null}),A())},i(o){n||(d(t),n=!0)},o(o){b(t),n=!1},d(o){o&&P(e),t&&t.d(o)}}}function ee(a){let e,n,t;function o(u){a[8](u)}let l={map:a[0],pmtiles:a[2]};return a[3]!==void 0&&(l.outputMetadata=a[3]),e=new $e({props:l}),j.push(()=>ne(e,"outputMetadata",o)),{c(){q(e.$$.fragment)},m(u,g){E(e,u,g),t=!0},p(u,g){const r={};g&1&&(r.map=u[0]),g&4&&(r.pmtiles=u[2]),!n&&g&8&&(n=!0,r.outputMetadata=u[3],ae(()=>n=!1)),e.$set(r)},i(u){t||(d(e.$$.fragment,u),t=!0)},o(u){b(e.$$.fragment,u),t=!1},d(u){O(e,u)}}}function Le(a){let e,n,t,o,l;function u(i){a[7](i)}let g={style:"https://api.maptiler.com/maps/dataviz/style.json?key=MZEJTanw3WpxRvt7qDfo",standardControls:!0,hash:!0,$$slots:{default:[Me]},$$scope:{ctx:a}};a[0]!==void 0&&(g.map=a[0]),n=new de({props:g}),j.push(()=>ne(n,"map",u));let r=a[0]&&ee(a);return{c(){e=_("div"),q(n.$$.fragment),o=N(),r&&r.c(),J(e,"slot","main"),D(e,"position","relative"),D(e,"width","100%"),D(e,"height","100vh")},m(i,p){S(i,e,p),E(n,e,null),m(e,o),r&&r.m(e,null),l=!0},p(i,p){const s={};p&2056&&(s.$$scope={dirty:p,ctx:i}),!t&&p&1&&(t=!0,s.map=i[0],ae(()=>t=!1)),n.$set(s),i[0]?r?(r.p(i,p),p&1&&d(r,1)):(r=ee(i),r.c(),d(r,1),r.m(e,null)):r&&(R(),b(r,1,1,()=>{r=null}),A())},i(i){l||(d(n.$$.fragment,i),d(r),l=!0)},o(i){b(n.$$.fragment,i),b(r),l=!1},d(i){i&&P(e),O(n),r&&r.d()}}}function Ce(a){let e,n;return e=new se({props:{$$slots:{main:[Le],left:[we]},$$scope:{ctx:a}}}),{c(){q(e.$$.fragment)},m(t,o){E(e,t,o),n=!0},p(t,[o]){const l={};o&2079&&(l.$$scope={dirty:o,ctx:t}),e.$set(l)},i(t){n||(d(e.$$.fragment,t),n=!0)},o(t){b(e.$$.fragment,t),n=!1},d(t){O(e,t)}}}function Se(a,e,n){pe(async()=>{await _e()});let t,o,l="",u,g={maxCount:1e3,originRadius:3,destinationRadius:3,streetviewOn:!1},r;function i(f){try{n(1,l="");let y=r.files;n(2,o=new Q(new ge(y[0])))}catch(y){window.alert(`Problem loading this PMTiles file. Don't open the GeoJSON file; make sure to select .pmtiles. Error: ${y}`)}}function p(f){t=f,n(0,t)}function s(f){u=f,n(3,u)}function k(f){j[f?"unshift":"push"](()=>{r=f,n(4,r)})}function $(){l=be(this),n(1,l)}return a.$$.update=()=>{a.$$.dirty&2&&l!=""&&n(2,o=new Q(`https://assets.od2net.org/output/${l}.pmtiles`)),a.$$.dirty&1&&t&&t.on("moveend",()=>{let f=[];for(let L of t.queryRenderedFeatures(void 0,{layers:["input-layer"]}))f.push(L.properties.count);let y=Math.min(...f),w=Math.max(...f),M=f.length;console.log({min:y,max:w,count:M})})},[t,l,o,u,r,g,i,p,s,k,$]}class Pe extends te{constructor(e){super(),oe(this,e,Se,Ce,T,{})}}new Pe({target:document.getElementById("app")});
