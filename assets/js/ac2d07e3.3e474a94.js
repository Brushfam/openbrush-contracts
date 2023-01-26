"use strict";(self.webpackChunkopenbrush=self.webpackChunkopenbrush||[]).push([[93570],{3905:(e,t,n)=>{n.d(t,{Zo:()=>c,kt:()=>f});var r=n(67294);function o(e,t,n){return t in e?Object.defineProperty(e,t,{value:n,enumerable:!0,configurable:!0,writable:!0}):e[t]=n,e}function a(e,t){var n=Object.keys(e);if(Object.getOwnPropertySymbols){var r=Object.getOwnPropertySymbols(e);t&&(r=r.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),n.push.apply(n,r)}return n}function s(e){for(var t=1;t<arguments.length;t++){var n=null!=arguments[t]?arguments[t]:{};t%2?a(Object(n),!0).forEach((function(t){o(e,t,n[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(n)):a(Object(n)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(n,t))}))}return e}function i(e,t){if(null==e)return{};var n,r,o=function(e,t){if(null==e)return{};var n,r,o={},a=Object.keys(e);for(r=0;r<a.length;r++)n=a[r],t.indexOf(n)>=0||(o[n]=e[n]);return o}(e,t);if(Object.getOwnPropertySymbols){var a=Object.getOwnPropertySymbols(e);for(r=0;r<a.length;r++)n=a[r],t.indexOf(n)>=0||Object.prototype.propertyIsEnumerable.call(e,n)&&(o[n]=e[n])}return o}var l=r.createContext({}),p=function(e){var t=r.useContext(l),n=t;return e&&(n="function"==typeof e?e(t):s(s({},t),e)),n},c=function(e){var t=p(e.components);return r.createElement(l.Provider,{value:t},e.children)},m="mdxType",u={inlineCode:"code",wrapper:function(e){var t=e.children;return r.createElement(r.Fragment,{},t)}},h=r.forwardRef((function(e,t){var n=e.components,o=e.mdxType,a=e.originalType,l=e.parentName,c=i(e,["components","mdxType","originalType","parentName"]),m=p(n),h=o,f=m["".concat(l,".").concat(h)]||m[h]||u[h]||a;return n?r.createElement(f,s(s({ref:t},c),{},{components:n})):r.createElement(f,s({ref:t},c))}));function f(e,t){var n=arguments,o=t&&t.mdxType;if("string"==typeof e||o){var a=n.length,s=new Array(a);s[0]=h;var i={};for(var l in t)hasOwnProperty.call(t,l)&&(i[l]=t[l]);i.originalType=e,i[m]="string"==typeof e?e:o,s[1]=i;for(var p=2;p<a;p++)s[p]=n[p];return r.createElement.apply(null,s)}return r.createElement.apply(null,n)}h.displayName="MDXCreateElement"},14107:(e,t,n)=>{n.r(t),n.d(t,{assets:()=>l,contentTitle:()=>s,default:()=>m,frontMatter:()=>a,metadata:()=>i,toc:()=>p});var r=n(87462),o=(n(67294),n(3905));const a={sidebar_position:5,title:"PSP22 FlashMint"},s=void 0,i={unversionedId:"smart-contracts/PSP22/Extensions/flashmint",id:"version-3.0.0-beta/smart-contracts/PSP22/Extensions/flashmint",title:"PSP22 FlashMint",description:"This example shows how you can reuse the implementation of PSP22 token with PSP22FlashMint extension, which allows the user to perform a flash loan on the token by minting the borrowed amount and then burning it along with fees for the loan.",source:"@site/versioned_docs/version-3.0.0-beta/smart-contracts/PSP22/Extensions/flashmint.md",sourceDirName:"smart-contracts/PSP22/Extensions",slug:"/smart-contracts/PSP22/Extensions/flashmint",permalink:"/smart-contracts/PSP22/Extensions/flashmint",draft:!1,editUrl:"https://github.com/727-Ventures/openbrush-contracts/tree/main/docs/versioned_docs/version-3.0.0-beta/smart-contracts/PSP22/Extensions/flashmint.md",tags:[],version:"3.0.0-beta",sidebarPosition:5,frontMatter:{sidebar_position:5,title:"PSP22 FlashMint"},sidebar:"tutorialSidebar",previous:{title:"PSP22 Wrapper",permalink:"/smart-contracts/PSP22/Extensions/wrapper"},next:{title:"PSP22 Pausable",permalink:"/smart-contracts/PSP22/Extensions/pausable"}},l={},p=[{value:"1. Implement the FlashMint extension",id:"1-implement-the-flashmint-extension",level:2}],c={toc:p};function m(e){let{components:t,...n}=e;return(0,o.kt)("wrapper",(0,r.Z)({},c,n,{components:t,mdxType:"MDXLayout"}),(0,o.kt)("p",null,"This example shows how you can reuse the implementation of ",(0,o.kt)("a",{parentName:"p",href:"https://github.com/727-Ventures/openbrush-contracts/tree/main/contracts/src/token/psp22"},"PSP22")," token with ",(0,o.kt)("a",{parentName:"p",href:"https://github.com/727-Ventures/openbrush-contracts/tree/main/contracts/src/token/psp22/extensions/flashmint.rs"},"PSP22FlashMint")," extension, which allows the user to perform a flash loan on the token by minting the borrowed amount and then burning it along with fees for the loan."),(0,o.kt)("h2",{id:"1-implement-the-flashmint-extension"},"1. Implement the FlashMint extension"),(0,o.kt)("p",null,"First, you should implement basic version of ",(0,o.kt)("a",{parentName:"p",href:"/smart-contracts/PSP22"},"PSP22"),"."),(0,o.kt)("p",null,"For your smart contract to use this extension, you need to implement the\n",(0,o.kt)("inlineCode",{parentName:"p"},"PSP22FlashMint")," trait in your ",(0,o.kt)("inlineCode",{parentName:"p"},"PSP22")," smart contract.\nImport everything from ",(0,o.kt)("inlineCode",{parentName:"p"},"openbrush::contracts::psp22::extensions::flashmint::*"),"\nand inherit the implementation for ",(0,o.kt)("inlineCode",{parentName:"p"},"PSP22FlashMint")," trait.\nYou can also customize (override) the original functions from ",(0,o.kt)("inlineCode",{parentName:"p"},"PSP22FlashMint"),"."),(0,o.kt)("pre",null,(0,o.kt)("code",{parentName:"pre",className:"language-rust"},"use openbrush::contracts::psp22::extensions::flashmint::*;\n\nimpl FlashLender for Contract {}\n")),(0,o.kt)("p",null,"And that's it! Your ",(0,o.kt)("inlineCode",{parentName:"p"},"PSP22")," is now extended by the ",(0,o.kt)("inlineCode",{parentName:"p"},"PSP22FlashMint")," extension and ready to use its functions!\nYou can check the full example of the implementation of this extension ",(0,o.kt)("a",{parentName:"p",href:"https://github.com/727-Ventures/openbrush-contracts/tree/main/examples/psp22_extensions/flashmint"},"here"),"."))}m.isMDXComponent=!0}}]);