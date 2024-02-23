import{S as ne,i as le,s as F,m as ie,P as re,L as se,c as E,a as O,t as _,b,d as q,o as ue,H as fe,e as g,f as N,g as T,h as j,j as M,k as pe,l as L,n as m,p as Q,q as U,r as A,u as B,v as C,w as me,M as ce,x as D,y as G,z as J,A as K,_ as _e,B as V,F as de,C as ge,D as ae,E as be}from"./Legend-71b0f321.js";import{S as he,L as ve}from"./SidebarControls-de35fcb0.js";let R="pmtilesSource";async function ke(a){let e=await a.getHeader(),o=await a.getMetadata();return{bounds:[e.minLon,e.minLat,e.maxLon,e.maxLat],outputMetadata:JSON.parse(o.description),minZoom:e.minZoom,maxZoom:e.maxZoom}}function ye(a,e,o){let{pmtiles:t}=e,{map:n}=e,{outputMetadata:l=null}=e,i=new re;ie.addProtocol("pmtiles",i.tile);async function f(s){r(R),i.add(s);let p=await ke(s);n.addSource(R,{type:"vector",tiles:["pmtiles://"+s.source.getKey()+"/{z}/{x}/{y}"],minzoom:p.minZoom,maxzoom:p.maxZoom,bounds:p.bounds}),n.fitBounds(p.bounds,{padding:100,duration:500}),o(0,l=p.outputMetadata)}function r(s){if(n.getSource(s)){let p=[];for(let u of n.getStyle().layers)"source"in u&&u.source==s&&p.push(u.id);for(let u of p)n.removeLayer(u);n.removeSource(s)}}return a.$$set=s=>{"pmtiles"in s&&o(1,t=s.pmtiles),"map"in s&&o(2,n=s.map),"outputMetadata"in s&&o(0,l=s.outputMetadata)},a.$$.update=()=>{a.$$.dirty&2&&(t?f(t):r(R))},[l,t,n]}class we extends ne{constructor(e){super(),le(this,e,ye,null,F,{pmtiles:1,map:2,outputMetadata:0})}}function X(a){let e,o,t;return{c(){e=T("Open a "),o=g("i"),o.textContent=".pmtiles",t=T(`
        file produced by the tool. Note this file stays in your browser; it doesn't
        get uploaded anywhere.`)},m(n,l){L(n,e,l),L(n,o,l),L(n,t,l)},d(n){n&&(C(e),C(o),C(t))}}}function x(a){let e,o,t;function n(i){a[11](i)}let l={outputMetadata:a[3],map:a[1]};return a[4]!==void 0&&(l.controls=a[4]),e=new he({props:l}),D.push(()=>G(e,"controls",n)),{c(){E(e.$$.fragment)},m(i,f){O(e,i,f),t=!0},p(i,f){const r={};f&8&&(r.outputMetadata=i[3]),f&2&&(r.map=i[1]),!o&&f&16&&(o=!0,r.controls=i[4],K(()=>o=!1)),e.$set(r)},i(i){t||(_(e.$$.fragment,i),t=!0)},o(i){b(e.$$.fragment,i),t=!1},d(i){q(e,i)}}}function $e(a){let e,o,t,n,l,i,f,r,s,p,u,y,w,$,d,k,S,P,Z,W,I,H,Y;o=new fe({props:{app:"main"}});let h=a[2]==null&&X(),c=a[3]&&x(a);return{c(){e=g("div"),E(o.$$.fragment),t=N(),n=g("label"),h&&h.c(),l=N(),i=g("input"),f=N(),r=g("div"),s=g("label"),p=T(`Or load an example:
        `),u=g("select"),y=g("option"),y.textContent="Custom file loaded",w=g("option"),w.textContent="Edinburgh",$=g("option"),$.textContent=`England (2011 home-to-work)
          `,d=g("option"),d.textContent="Liverpool (direct)",k=g("option"),k.textContent="Liverpool (quiet)",S=g("option"),S.textContent="London",P=g("option"),P.textContent="Seattle",Z=g("option"),Z.textContent="York",W=N(),c&&c.c(),j(i,"type","file"),y.__value="",M(y,y.__value),w.__value="edinburgh",M(w,w.__value),$.__value="england_2011_home_to_work",M($,$.__value),d.__value="liverpool_direct",M(d,d.__value),k.__value="liverpool_quiet",M(k,k.__value),S.__value="london",M(S,S.__value),P.__value="seattle",M(P,P.__value),Z.__value="york",M(Z,Z.__value),a[0]===void 0&&pe(()=>a[10].call(u)),j(e,"slot","left")},m(v,z){L(v,e,z),O(o,e,null),m(e,t),m(e,n),h&&h.m(n,null),m(n,l),m(n,i),a[9](i),m(e,f),m(e,r),m(r,s),m(s,p),m(s,u),m(u,y),m(u,w),m(u,$),m(u,d),m(u,k),m(u,S),m(u,P),m(u,Z),Q(u,a[0],!0),m(e,W),c&&c.m(e,null),I=!0,H||(Y=[U(i,"change",a[6]),U(u,"change",a[10])],H=!0)},p(v,z){v[2]==null?h||(h=X(),h.c(),h.m(n,l)):h&&(h.d(1),h=null),z&1&&Q(u,v[0]),v[3]?c?(c.p(v,z),z&8&&_(c,1)):(c=x(v),c.c(),_(c,1),c.m(e,null)):c&&(A(),b(c,1,1,()=>{c=null}),B())},i(v){I||(_(o.$$.fragment,v),_(c),I=!0)},o(v){b(o.$$.fragment,v),b(c),I=!1},d(v){v&&C(e),q(o),h&&h.d(),a[9](null),c&&c.d(),H=!1,me(Y)}}}function ee(a){let e=a[3],o,t,n=te(a);return{c(){n.c(),o=ae()},m(l,i){n.m(l,i),L(l,o,i),t=!0},p(l,i){i&8&&F(e,e=l[3])?(A(),b(n,1,1,be),B(),n=te(l),n.c(),_(n,1),n.m(o.parentNode,o)):n.p(l,i)},i(l){t||(_(n),t=!0)},o(l){b(n),t=!1},d(l){l&&C(o),n.d(l)}}}function te(a){let e,o;return e=new ve({props:{sourceOverride:{source:"pmtilesSource",sourceLayer:"rnet"},controls:a[4]}}),{c(){E(e.$$.fragment)},m(t,n){O(e,t,n),o=!0},p(t,n){const l={};n&16&&(l.controls=t[4]),e.$set(l)},i(t){o||(_(e.$$.fragment,t),o=!0)},o(t){b(e.$$.fragment,t),o=!1},d(t){q(e,t)}}}function Me(a){let e,o,t=a[3]&&ee(a);return{c(){t&&t.c(),e=ae()},m(n,l){t&&t.m(n,l),L(n,e,l),o=!0},p(n,l){n[3]?t?(t.p(n,l),l&8&&_(t,1)):(t=ee(n),t.c(),_(t,1),t.m(e.parentNode,e)):t&&(A(),b(t,1,1,()=>{t=null}),B())},i(n){o||(_(t),o=!0)},o(n){b(t),o=!1},d(n){n&&C(e),t&&t.d(n)}}}function oe(a){let e,o,t;function n(i){a[8](i)}let l={map:a[1],pmtiles:a[2]};return a[3]!==void 0&&(l.outputMetadata=a[3]),e=new we({props:l}),D.push(()=>G(e,"outputMetadata",n)),{c(){E(e.$$.fragment)},m(i,f){O(e,i,f),t=!0},p(i,f){const r={};f&2&&(r.map=i[1]),f&4&&(r.pmtiles=i[2]),!o&&f&8&&(o=!0,r.outputMetadata=i[3],K(()=>o=!1)),e.$set(r)},i(i){t||(_(e.$$.fragment,i),t=!0)},o(i){b(e.$$.fragment,i),t=!1},d(i){q(e,i)}}}function Le(a){let e,o,t,n,l;function i(s){a[7](s)}let f={style:"https://api.maptiler.com/maps/dataviz/style.json?key=MZEJTanw3WpxRvt7qDfo",standardControls:!0,hash:!0,$$slots:{default:[Me]},$$scope:{ctx:a}};a[1]!==void 0&&(f.map=a[1]),o=new ce({props:f}),D.push(()=>G(o,"map",i));let r=a[1]&&oe(a);return{c(){e=g("div"),E(o.$$.fragment),n=N(),r&&r.c(),j(e,"slot","main"),J(e,"position","relative"),J(e,"width","100%"),J(e,"height","100vh")},m(s,p){L(s,e,p),O(o,e,null),m(e,n),r&&r.m(e,null),l=!0},p(s,p){const u={};p&4120&&(u.$$scope={dirty:p,ctx:s}),!t&&p&2&&(t=!0,u.map=s[1],K(()=>t=!1)),o.$set(u),s[1]?r?(r.p(s,p),p&2&&_(r,1)):(r=oe(s),r.c(),_(r,1),r.m(e,null)):r&&(A(),b(r,1,1,()=>{r=null}),B())},i(s){l||(_(o.$$.fragment,s),_(r),l=!0)},o(s){b(o.$$.fragment,s),b(r),l=!1},d(s){s&&C(e),q(o),r&&r.d()}}}function Ce(a){let e,o;return e=new se({props:{$$slots:{main:[Le],left:[$e]},$$scope:{ctx:a}}}),{c(){E(e.$$.fragment)},m(t,n){O(e,t,n),o=!0},p(t,[n]){const l={};n&4159&&(l.$$scope={dirty:n,ctx:t}),e.$set(l)},i(t){o||(_(e.$$.fragment,t),o=!0)},o(t){b(e.$$.fragment,t),o=!1},d(t){q(e,t)}}}function Se(a,e,o){ue(async()=>{await _e()});let t,n,l="",i,f={maxCount:1e3,originRadius:3,destinationRadius:3,streetviewOn:!1},r;function s(d){try{o(0,l="");let k=r.files;o(2,n=new V(new de(k[0])))}catch(k){window.alert(`Problem loading this PMTiles file. Don't open the GeoJSON file; make sure to select .pmtiles. Error: ${k}`)}}function p(d){t=d,o(1,t)}function u(d){i=d,o(3,i)}function y(d){D[d?"unshift":"push"](()=>{r=d,o(5,r)})}function w(){l=ge(this),o(0,l)}function $(d){f=d,o(4,f)}return a.$$.update=()=>{a.$$.dirty&1&&l!=""&&o(2,n=new V(`https://assets.od2net.org/output/${l}.pmtiles`))},[l,t,n,i,f,r,s,p,u,y,w,$]}class Pe extends ne{constructor(e){super(),le(this,e,Se,Ce,F,{})}}new Pe({target:document.getElementById("app")});
