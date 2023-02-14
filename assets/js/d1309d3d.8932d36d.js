"use strict";(self.webpackChunkopenbrush=self.webpackChunkopenbrush||[]).push([[87599],{3905:(e,t,n)=>{n.d(t,{Zo:()=>c,kt:()=>b});var r=n(67294);function a(e,t,n){return t in e?Object.defineProperty(e,t,{value:n,enumerable:!0,configurable:!0,writable:!0}):e[t]=n,e}function o(e,t){var n=Object.keys(e);if(Object.getOwnPropertySymbols){var r=Object.getOwnPropertySymbols(e);t&&(r=r.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),n.push.apply(n,r)}return n}function s(e){for(var t=1;t<arguments.length;t++){var n=null!=arguments[t]?arguments[t]:{};t%2?o(Object(n),!0).forEach((function(t){a(e,t,n[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(n)):o(Object(n)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(n,t))}))}return e}function i(e,t){if(null==e)return{};var n,r,a=function(e,t){if(null==e)return{};var n,r,a={},o=Object.keys(e);for(r=0;r<o.length;r++)n=o[r],t.indexOf(n)>=0||(a[n]=e[n]);return a}(e,t);if(Object.getOwnPropertySymbols){var o=Object.getOwnPropertySymbols(e);for(r=0;r<o.length;r++)n=o[r],t.indexOf(n)>=0||Object.prototype.propertyIsEnumerable.call(e,n)&&(a[n]=e[n])}return a}var p=r.createContext({}),l=function(e){var t=r.useContext(p),n=t;return e&&(n="function"==typeof e?e(t):s(s({},t),e)),n},c=function(e){var t=l(e.components);return r.createElement(p.Provider,{value:t},e.children)},u="mdxType",m={inlineCode:"code",wrapper:function(e){var t=e.children;return r.createElement(r.Fragment,{},t)}},d=r.forwardRef((function(e,t){var n=e.components,a=e.mdxType,o=e.originalType,p=e.parentName,c=i(e,["components","mdxType","originalType","parentName"]),u=l(n),d=a,b=u["".concat(p,".").concat(d)]||u[d]||m[d]||o;return n?r.createElement(b,s(s({ref:t},c),{},{components:n})):r.createElement(b,s({ref:t},c))}));function b(e,t){var n=arguments,a=t&&t.mdxType;if("string"==typeof e||a){var o=n.length,s=new Array(o);s[0]=d;var i={};for(var p in t)hasOwnProperty.call(t,p)&&(i[p]=t[p]);i.originalType=e,i[u]="string"==typeof e?e:a,s[1]=i;for(var l=2;l<o;l++)s[l]=n[l];return r.createElement.apply(null,s)}return r.createElement.apply(null,n)}d.displayName="MDXCreateElement"},86130:(e,t,n)=>{n.r(t),n.d(t,{assets:()=>p,contentTitle:()=>s,default:()=>m,frontMatter:()=>o,metadata:()=>i,toc:()=>l});var r=n(87462),a=(n(67294),n(3905));const o={sidebar_position:1,title:"PSP37 Enumerable"},s=void 0,i={unversionedId:"smart-contracts/PSP37/Extensions/enumerable",id:"version-v2.3.0/smart-contracts/PSP37/Extensions/enumerable",title:"PSP37 Enumerable",description:"This example shows how you can reuse the implementation of PSP37 token with PSP37Enumerable extension.",source:"@site/versioned_docs/version-v2.3.0/smart-contracts/PSP37/Extensions/enumerable.md",sourceDirName:"smart-contracts/PSP37/Extensions",slug:"/smart-contracts/PSP37/Extensions/enumerable",permalink:"/openbrush-contracts/v2.3.0/smart-contracts/PSP37/Extensions/enumerable",draft:!1,editUrl:"https://github.com/727-Ventures/openbrush-contracts/tree/main/docs/versioned_docs/version-v2.3.0/smart-contracts/PSP37/Extensions/enumerable.md",tags:[],version:"v2.3.0",sidebarPosition:1,frontMatter:{sidebar_position:1,title:"PSP37 Enumerable"},sidebar:"tutorialSidebar",previous:{title:"PSP37",permalink:"/openbrush-contracts/v2.3.0/smart-contracts/PSP37/"},next:{title:"PSP37 Metadata",permalink:"/openbrush-contracts/v2.3.0/smart-contracts/PSP37/Extensions/metadata"}},p={},l=[{value:"Step 1: Add imports and enable unstable feature",id:"step-1-add-imports-and-enable-unstable-feature",level:2},{value:"Step 2: Define storage",id:"step-2-define-storage",level:2},{value:"Step 3: Inherit logic",id:"step-3-inherit-logic",level:2},{value:"Final code",id:"final-code",level:2}],c={toc:l},u="wrapper";function m(e){let{components:t,...n}=e;return(0,a.kt)(u,(0,r.Z)({},c,n,{components:t,mdxType:"MDXLayout"}),(0,a.kt)("p",null,"This example shows how you can reuse the implementation of ",(0,a.kt)("a",{parentName:"p",href:"https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/token/psp37"},"PSP37")," token with ",(0,a.kt)("a",{parentName:"p",href:"https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/token/psp37/extensions/enumerable.rs"},"PSP37Enumerable")," extension."),(0,a.kt)("p",null,"First, you should implement basic version of ",(0,a.kt)("a",{parentName:"p",href:"/smart-contracts/PSP37"},"PSP37"),"."),(0,a.kt)("h2",{id:"step-1-add-imports-and-enable-unstable-feature"},"Step 1: Add imports and enable unstable feature"),(0,a.kt)("p",null,"Import ",(0,a.kt)("strong",{parentName:"p"},"everything")," from ",(0,a.kt)("inlineCode",{parentName:"p"},"openbrush::contracts::psp37::extensions::enumerable"),"."),(0,a.kt)("pre",null,(0,a.kt)("code",{parentName:"pre",className:"language-rust"},'#![cfg_attr(not(feature = "std"), no_std)]\n#![feature(min_specialization)]\n\n#[openbrush::contract]\npub mod my_psp37 {\n    use openbrush::contracts::psp37::extensions::enumerable::*;\n...\n')),(0,a.kt)("h2",{id:"step-2-define-storage"},"Step 2: Define storage"),(0,a.kt)("p",null,"Pass ",(0,a.kt)("inlineCode",{parentName:"p"},"enumerable::Balances")," into ",(0,a.kt)("inlineCode",{parentName:"p"},"psp37::Data")," to be able to use ",(0,a.kt)("inlineCode",{parentName:"p"},"PSP37Enumerable")," extension in your ",(0,a.kt)("inlineCode",{parentName:"p"},"PSP37")," implementation."),(0,a.kt)("pre",null,(0,a.kt)("code",{parentName:"pre",className:"language-rust"},"#[derive(Default, SpreadAllocate, Storage)]\n#[ink(storage)]\npub struct Contract {\n    #[storage_field]\n    psp37: psp37::Data<enumerable::Balances>,\n}\n")),(0,a.kt)("h2",{id:"step-3-inherit-logic"},"Step 3: Inherit logic"),(0,a.kt)("p",null,"Inherit implementation of the ",(0,a.kt)("inlineCode",{parentName:"p"},"PSP37Enumerable")," trait. You can customize (override) methods in this ",(0,a.kt)("inlineCode",{parentName:"p"},"impl")," block."),(0,a.kt)("pre",null,(0,a.kt)("code",{parentName:"pre",className:"language-rust"},"\nimpl PSP37 for Contract {}\n\nimpl PSP37Enumerable for Contract {}\n")),(0,a.kt)("h2",{id:"final-code"},"Final code"),(0,a.kt)("pre",null,(0,a.kt)("code",{parentName:"pre",className:"language-rust"},'#![cfg_attr(not(feature = "std"), no_std)]\n#![feature(min_specialization)]\n\n#[openbrush::contract]\npub mod my_psp37_enumerable {\n    use ink_storage::traits::SpreadAllocate;\n    use openbrush::{\n        contracts::psp37::extensions::enumerable::*,\n        traits::Storage,\n    };\n\n    #[derive(Default, SpreadAllocate, Storage)]\n    #[ink(storage)]\n    pub struct Contract {\n        #[storage_field]\n        psp37: psp37::Data<enumerable::Balances>,\n    }\n\n    impl PSP37 for Contract {}\n    impl PSP37Enumerable for Contract {}\n\n    impl Contract {\n        #[ink(constructor)]\n        pub fn new() -> Self {\n            ink_lang::codegen::initialize_contract(|_instance: &mut Self| {})\n        }\n    }\n}\n')),(0,a.kt)("p",null,"And that's it! Your ",(0,a.kt)("inlineCode",{parentName:"p"},"PSP37")," is now extended by the ",(0,a.kt)("inlineCode",{parentName:"p"},"PSP37Enumerable")," extension and ready to use its functions!\nYou can check an example of the usage of ",(0,a.kt)("a",{parentName:"p",href:"https://github.com/Supercolony-net/openbrush-contracts/tree/main/examples/psp37_extensions/enumerable"},"PSP37 Enumerable"),"."),(0,a.kt)("p",null,"You can also check the documentation for the basic implementation of ",(0,a.kt)("a",{parentName:"p",href:"/smart-contracts/PSP37"},"PSP37"),"."))}m.isMDXComponent=!0}}]);