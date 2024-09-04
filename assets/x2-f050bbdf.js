import{S as ie,i as re,s as fe,G as _e,e as E,f as T,A as H,g as O,I as $,u as J,J as V,K as be,N as he,O as we,Q as Le,R as ye,t as h,b as k,h as q,x as ke,T as ve,U as Ce,V as Y,W as ee,m as Me,L as Be,c as P,j as A,a as z,d as I,o as Se,H as Ee,n as ue,y as Z,z as U,k as x,q as B,B as K,v as te,w as ne,M as Pe,_ as ze,X as ce,l as ae,E as Ie,Y as Oe}from"./Layout-af65b6e0.js";import{L as qe,C as Re,O as Te,_ as Ae,J as De,a as Je,G as je,P as Fe}from"./index-7c7636be.js";import{S as Ne,L as Xe}from"./SidebarControls-dca6fc95.js";const Ze=a=>({marker:a&8}),se=a=>({marker:a[3]});function Ge(a){let e,t,s,n,i,r,d;const l=a[19].default,f=_e(l,a,a[18],se);return{c(){e=E("div"),f&&f.c(),T(e,"tabindex",t=a[1]?0:void 0),T(e,"role",s=a[1]?"button":void 0),H(e,"z-index",a[2])},m(o,c){O(o,e,c),f&&f.m(e,null),i=!0,r||(d=[$(a[7].call(null,e)),$(n=He.call(null,e,a[0])),J(e,"click",V(a[20])),J(e,"dblclick",V(a[21])),J(e,"contextmenu",V(be(a[22]))),J(e,"mouseenter",a[23]),J(e,"mouseleave",a[24]),J(e,"mousemove",a[25]),J(e,"keydown",a[8])],r=!0)},p(o,[c]){f&&f.p&&(!i||c&262152)&&he(f,l,o,o[18],i?Le(l,o[18],c,Ze):we(o[18]),se),(!i||c&2&&t!==(t=o[1]?0:void 0))&&T(e,"tabindex",t),(!i||c&2&&s!==(s=o[1]?"button":void 0))&&T(e,"role",s),n&&ye(n.update)&&c&1&&n.update.call(null,o[0]),c&4&&H(e,"z-index",o[2])},i(o){i||(h(f,o),i=!0)},o(o){k(f,o),i=!1},d(o){o&&q(e),f&&f.d(o),r=!1,ke(d)}}}function He(a,e){const t=a.className;function s(n){n?a.className=`${t} ${n}`:a.className=t}return s(e),{update:s}}function Ue(a,e,t){let s,n,i,{$$slots:r={},$$scope:d}=e,{marker:l=void 0}=e,{lngLat:f}=e,{class:o=void 0}=e,{interactive:c=!0}=e,{asButton:w=!1}=e,{draggable:L=!1}=e,{feature:y=null}=e,{offset:g=void 0}=e,{zIndex:v=void 0}=e,{rotation:R=0}=e,{opacity:D=1}=e;const b=ve(),{map:_,layerEvent:m,self:M,markerClickManager:j}=Ce();Y(a,_,u=>t(27,i=u)),Y(a,m,u=>t(26,s=u)),Y(a,M,u=>t(3,n=u));function F(u){ee(M,n=new Me.Marker({element:u,rotation:R,draggable:L,offset:g,opacity:D.toString()}).setLngLat(f).addTo(i),n),t(11,l=n);const N=()=>S("dragstart"),G=()=>{W(),S("drag")},X=()=>{W(),S("dragend")};return L&&(n.on("dragstart",N),n.on("drag",G),n.on("dragend",X)),{destroy(){L&&(n==null||n.off("dragstart",N),n==null||n.off("drag",G),n==null||n.off("dragend",X)),t(11,l=void 0),n==null||n.remove()}}}function W(){let u=n==null?void 0:n.getLngLat();u&&(Array.isArray(f)?t(10,f=[u.lng,u.lat]):f&&"lon"in f?t(10,f={lon:u.lng,lat:u.lat}):t(10,f=u))}function Q(u){u.key===" "&&(u.preventDefault(),u.stopPropagation(),S("click"))}function S(u){if(!c)return;let N=n==null?void 0:n.getLngLat();if(!N)return;const G=[N.lng,N.lat];let X={map:i,marker:n,lngLat:G,features:[{type:"Feature",properties:(y==null?void 0:y.properties)??{},geometry:{type:"Point",coordinates:G}}]};(u==="click"||u==="contextmenu")&&j.handleClick(X),ee(m,s={...X,layerType:"marker",type:u},s),b(u,X)}const p=()=>S("click"),C=()=>S("dblclick"),de=()=>{S("contextmenu")},pe=()=>{S("mouseenter")},ge=()=>{S("mouseleave")},me=()=>S("mousemove");return a.$$set=u=>{"marker"in u&&t(11,l=u.marker),"lngLat"in u&&t(10,f=u.lngLat),"class"in u&&t(0,o=u.class),"interactive"in u&&t(12,c=u.interactive),"asButton"in u&&t(1,w=u.asButton),"draggable"in u&&t(13,L=u.draggable),"feature"in u&&t(14,y=u.feature),"offset"in u&&t(15,g=u.offset),"zIndex"in u&&t(2,v=u.zIndex),"rotation"in u&&t(16,R=u.rotation),"opacity"in u&&t(17,D=u.opacity),"$$scope"in u&&t(18,d=u.$$scope)},a.$$.update=()=>{a.$$.dirty&1032&&(n==null||n.setLngLat(f)),a.$$.dirty&32776&&(n==null||n.setOffset(g??[0,0])),a.$$.dirty&65544&&(n==null||n.setRotation(R)),a.$$.dirty&131080&&(n==null||n.setOpacity(D.toString()))},[o,w,v,n,_,m,M,F,Q,S,f,l,c,L,y,g,R,D,d,r,p,C,de,pe,ge,me]}class Ke extends ie{constructor(e){super(),re(this,e,Ue,Ge,fe,{marker:11,lngLat:10,class:0,interactive:12,asButton:1,draggable:13,feature:14,offset:15,zIndex:2,rotation:16,opacity:17})}}const We=`<svg width="40" height="50" viewBox="0 0 40 50" xmlns="http://www.w3.org/2000/svg">
    <defs/>
    <path id="path2" fill="none" stroke="none" visibility="hidden" d="M 1.0169492 25.59322 L 25.016949 25.59322 L 25.016949 49.59322 L 1.0169492 49.59322 Z"/>
    <path id="path4" fill="#000000" stroke="none" d="M 19.198305 2.6101693 C 10.336661 2.6101693 3.1694916 9.7773384 3.1694916 18.638983 C 3.1694916 30.660593 19.198305 48.406779 19.198305 48.406779 C 19.198305 48.406779 35.227118 30.660593 35.227118 18.638983 C 35.227118 9.7773384 28.059949 2.6101693 19.198305 2.6101693 Z M 19.198305 24.363559 C 16.038339 24.363559 13.473728 21.798949 13.473728 18.638983 C 13.473728 15.479016 16.038339 12.914406 19.198305 12.914406 C 22.358271 12.914406 24.922881 15.479016 24.922881 18.638983 C 24.922881 21.798949 22.358271 24.363559 19.198305 24.363559 Z"/>
</svg>
`;function Qe(a){let e,t,s;return t=new Ee({props:{app:"interactive"}}),{c(){e=E("div"),P(t.$$.fragment),T(e,"slot","top")},m(n,i){O(n,e,i),z(t,e,null),s=!0},p:ue,i(n){s||(h(t.$$.fragment,n),s=!0)},o(n){k(t.$$.fragment,n),s=!1},d(n){n&&q(e),I(t)}}}function oe(a){let e,t,s,n,i,r,d,l,f,o,c,w;function L(g){a[20](g)}let y={};return a[2]!==void 0&&(y.cost=a[2]),l=new Je({props:y}),Z.push(()=>U(l,"cost",L)),{c(){e=E("div"),t=E("label"),s=x(`Max requests (limit for faster updates):
          `),n=E("br"),i=A(),r=E("input"),d=A(),P(l.$$.fragment),T(r,"type","number"),T(r,"min",1)},m(g,v){O(g,e,v),B(e,t),B(t,s),B(t,n),B(t,i),B(t,r),ae(r,a[1]),O(g,d,v),z(l,g,v),o=!0,c||(w=J(r,"input",a[19]),c=!0)},p(g,v){v&2&&ce(r.value)!==g[1]&&ae(r,g[1]);const R={};!f&&v&4&&(f=!0,R.cost=g[2],K(()=>f=!1)),l.$set(R)},i(g){o||(h(l.$$.fragment,g),o=!0)},o(g){k(l.$$.fragment,g),o=!1},d(g){g&&(q(e),q(d)),I(l,g),c=!1,w()}}}function le(a){let e,t,s,n,i;function r(l){a[21](l)}let d={outputMetadata:a[6].metadata,map:a[3]};return a[8]!==void 0&&(d.controls=a[8]),s=new Ne({props:d}),Z.push(()=>U(s,"controls",r)),{c(){e=E("hr"),t=A(),P(s.$$.fragment)},m(l,f){O(l,e,f),O(l,t,f),z(s,l,f),i=!0},p(l,f){const o={};f&64&&(o.outputMetadata=l[6].metadata),f&8&&(o.map=l[3]),!n&&f&256&&(n=!0,o.controls=l[8],K(()=>n=!1)),s.$set(o)},i(l){i||(h(s.$$.fragment,l),i=!0)},o(l){k(s.$$.fragment,l),i=!1},d(l){l&&(q(e),q(t)),I(s,l)}}}function Ve(a){let e,t,s,n,i,r,d,l,f,o,c,w,L,y,g,v;function R(m){a[16](m)}let D={};a[0]!==void 0&&(D.example=a[0]),l=new Re({props:D}),Z.push(()=>U(l,"example",R)),c=new Te({props:{map:a[3]}}),c.$on("gotXml",a[12]),c.$on("loading",a[17]),c.$on("error",a[18]);let b=a[4]&&oe(a),_=a[6].metadata&&le(a);return{c(){e=E("div"),t=E("label"),s=x("Open an "),n=E("i"),n.textContent=".osm.pbf",i=x(`
      file
      `),r=E("input"),d=A(),P(l.$$.fragment),o=A(),P(c.$$.fragment),w=A(),b&&b.c(),L=A(),_&&_.c(),T(r,"type","file"),T(e,"slot","left")},m(m,M){O(m,e,M),B(e,t),B(t,s),B(t,n),B(t,i),B(t,r),a[15](r),B(e,d),z(l,e,null),B(e,o),z(c,e,null),B(e,w),b&&b.m(e,null),B(e,L),_&&_.m(e,null),y=!0,g||(v=J(r,"change",a[10]),g=!0)},p(m,M){const j={};!f&&M&1&&(f=!0,j.example=m[0],K(()=>f=!1)),l.$set(j);const F={};M&8&&(F.map=m[3]),c.$set(F),m[4]?b?(b.p(m,M),M&16&&h(b,1)):(b=oe(m),b.c(),h(b,1),b.m(e,L)):b&&(te(),k(b,1,1,()=>{b=null}),ne()),m[6].metadata?_?(_.p(m,M),M&64&&h(_,1)):(_=le(m),_.c(),h(_,1),_.m(e,null)):_&&(te(),k(_,1,1,()=>{_=null}),ne())},i(m){y||(h(l.$$.fragment,m),h(c.$$.fragment,m),h(b),h(_),y=!0)},o(m){k(l.$$.fragment,m),k(c.$$.fragment,m),k(b),k(_),y=!1},d(m){m&&q(e),a[15](null),I(l),I(c),b&&b.d(),_&&_.d(),g=!1,v()}}}function Ye(a){let e,t;return{c(){e=new Oe(!1),t=Ie(),e.a=t},m(s,n){e.m(We,s,n),O(s,t,n)},p:ue,d(s){s&&(q(t),e.d())}}}function xe(a){let e,t;return e=new Xe({props:{controls:a[8]}}),{c(){P(e.$$.fragment)},m(s,n){z(e,s,n),t=!0},p(s,n){const i={};n&256&&(i.controls=s[8]),e.$set(i)},i(s){t||(h(e.$$.fragment,s),t=!0)},o(s){k(e.$$.fragment,s),t=!1},d(s){I(e,s)}}}function $e(a){let e,t,s,n,i,r,d;function l(o){a[13](o)}let f={draggable:!0,$$slots:{default:[Ye]},$$scope:{ctx:a}};return a[5]!==void 0&&(f.lngLat=a[5]),e=new Ke({props:f}),Z.push(()=>U(e,"lngLat",l)),e.$on("dragend",a[11]),n=new je({props:{data:a[6],$$slots:{default:[xe]},$$scope:{ctx:a}}}),r=new Fe({}),{c(){P(e.$$.fragment),s=A(),P(n.$$.fragment),i=A(),P(r.$$.fragment)},m(o,c){z(e,o,c),O(o,s,c),z(n,o,c),O(o,i,c),z(r,o,c),d=!0},p(o,c){const w={};c&33554432&&(w.$$scope={dirty:c,ctx:o}),!t&&c&32&&(t=!0,w.lngLat=o[5],K(()=>t=!1)),e.$set(w);const L={};c&64&&(L.data=o[6]),c&33554688&&(L.$$scope={dirty:c,ctx:o}),n.$set(L)},i(o){d||(h(e.$$.fragment,o),h(n.$$.fragment,o),h(r.$$.fragment,o),d=!0)},o(o){k(e.$$.fragment,o),k(n.$$.fragment,o),k(r.$$.fragment,o),d=!1},d(o){o&&(q(s),q(i)),I(e,o),I(n,o),I(r,o)}}}function et(a){let e,t,s,n;function i(d){a[14](d)}let r={style:"https://api.maptiler.com/maps/dataviz/style.json?key=MZEJTanw3WpxRvt7qDfo",standardControls:!0,hash:!0,$$slots:{default:[$e]},$$scope:{ctx:a}};return a[3]!==void 0&&(r.map=a[3]),t=new Pe({props:r}),Z.push(()=>U(t,"map",i)),{c(){e=E("div"),P(t.$$.fragment),T(e,"slot","main"),H(e,"position","relative"),H(e,"width","100%"),H(e,"height","100vh")},m(d,l){O(d,e,l),z(t,e,null),n=!0},p(d,l){const f={};l&33554784&&(f.$$scope={dirty:l,ctx:d}),!s&&l&8&&(s=!0,f.map=d[3],K(()=>s=!1)),t.$set(f)},i(d){n||(h(t.$$.fragment,d),n=!0)},o(d){k(t.$$.fragment,d),n=!1},d(d){d&&q(e),I(t)}}}function tt(a){let e,t,s,n;return e=new Be({props:{$$slots:{main:[et],left:[Ve],top:[Qe]},$$scope:{ctx:a}}}),s=new qe({props:{loading:a[7]}}),{c(){P(e.$$.fragment),t=A(),P(s.$$.fragment)},m(i,r){z(e,i,r),O(i,t,r),z(s,i,r),n=!0},p(i,[r]){const d={};r&33555455&&(d.$$scope={dirty:r,ctx:i}),e.$set(d);const l={};r&128&&(l.loading=i[7]),s.$set(l)},i(i){n||(h(e.$$.fragment,i),h(s.$$.fragment,i),n=!0)},o(i){k(e.$$.fragment,i),k(s.$$.fragment,i),n=!1},d(i){i&&q(t),I(e,i),I(s,i)}}}function nt(a,e,t){Se(async()=>{await Ae(),await ze()});let s,n,i="",r={lng:0,lat:0},d={type:"FeatureCollection",features:[]},l="",f=1e3,o="Distance",c={maxCount:1e3,originRadius:3,destinationRadius:3,streetviewOn:!1},w;async function L(p){t(0,i=""),t(7,l="Loading file"),y(await w.files[0].arrayBuffer())}function y(p){try{t(4,n=new De(new Uint8Array(p))),t(2,o="Distance");let C=n.getBounds();s.fitBounds([[C[0],C[1]],[C[2],C[3]]],{padding:20,animate:!1}),t(5,r.lng=(C[0]+C[2])/2,r),t(5,r.lat=(C[1]+C[3])/2,r),v()}catch(C){window.alert(`Problem importing osm.pbf file: ${C}`)}t(7,l="")}async function g(p){if(p!=""){t(7,l=`Loading ${p}`);let C=await fetch(`https://assets.od2net.org/pbf_clips/${p}.osm.pbf`);y(await C.arrayBuffer())}}function v(){n&&t(6,d=JSON.parse(n.recalculate({lng:r.lng,lat:r.lat,max_requests:f,cost:o})))}function R(p,C){v()}function D(p){t(7,l="Parsing XML"),y(new TextEncoder().encode(p.detail)),t(7,l="")}function b(p){r=p,t(5,r)}function _(p){s=p,t(3,s)}function m(p){Z[p?"unshift":"push"](()=>{w=p,t(9,w)})}function M(p){i=p,t(0,i)}const j=p=>t(7,l=p.detail),F=p=>t(7,l=p.detail);function W(){f=ce(this.value),t(1,f)}function Q(p){o=p,t(2,o)}function S(p){c=p,t(8,c)}return a.$$.update=()=>{a.$$.dirty&1&&g(i),a.$$.dirty&6&&R()},[i,f,o,s,n,r,d,l,c,w,L,v,D,b,_,m,M,j,F,W,Q,S]}class at extends ie{constructor(e){super(),re(this,e,nt,tt,fe,{})}}new at({target:document.getElementById("app")});