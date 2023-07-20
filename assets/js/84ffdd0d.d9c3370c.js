"use strict";(self.webpackChunkopenbrush=self.webpackChunkopenbrush||[]).push([[17808],{3905:(e,t,n)=>{n.d(t,{Zo:()=>l,kt:()=>f});var a=n(67294);function r(e,t,n){return t in e?Object.defineProperty(e,t,{value:n,enumerable:!0,configurable:!0,writable:!0}):e[t]=n,e}function o(e,t){var n=Object.keys(e);if(Object.getOwnPropertySymbols){var a=Object.getOwnPropertySymbols(e);t&&(a=a.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),n.push.apply(n,a)}return n}function s(e){for(var t=1;t<arguments.length;t++){var n=null!=arguments[t]?arguments[t]:{};t%2?o(Object(n),!0).forEach((function(t){r(e,t,n[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(n)):o(Object(n)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(n,t))}))}return e}function i(e,t){if(null==e)return{};var n,a,r=function(e,t){if(null==e)return{};var n,a,r={},o=Object.keys(e);for(a=0;a<o.length;a++)n=o[a],t.indexOf(n)>=0||(r[n]=e[n]);return r}(e,t);if(Object.getOwnPropertySymbols){var o=Object.getOwnPropertySymbols(e);for(a=0;a<o.length;a++)n=o[a],t.indexOf(n)>=0||Object.prototype.propertyIsEnumerable.call(e,n)&&(r[n]=e[n])}return r}var p=a.createContext({}),c=function(e){var t=a.useContext(p),n=t;return e&&(n="function"==typeof e?e(t):s(s({},t),e)),n},l=function(e){var t=c(e.components);return a.createElement(p.Provider,{value:t},e.children)},u="mdxType",m={inlineCode:"code",wrapper:function(e){var t=e.children;return a.createElement(a.Fragment,{},t)}},d=a.forwardRef((function(e,t){var n=e.components,r=e.mdxType,o=e.originalType,p=e.parentName,l=i(e,["components","mdxType","originalType","parentName"]),u=c(n),d=r,f=u["".concat(p,".").concat(d)]||u[d]||m[d]||o;return n?a.createElement(f,s(s({ref:t},l),{},{components:n})):a.createElement(f,s({ref:t},l))}));function f(e,t){var n=arguments,r=t&&t.mdxType;if("string"==typeof e||r){var o=n.length,s=new Array(o);s[0]=d;var i={};for(var p in t)hasOwnProperty.call(t,p)&&(i[p]=t[p]);i.originalType=e,i[u]="string"==typeof e?e:r,s[1]=i;for(var c=2;c<o;c++)s[c]=n[c];return a.createElement.apply(null,s)}return a.createElement.apply(null,n)}d.displayName="MDXCreateElement"},95185:(e,t,n)=>{n.r(t),n.d(t,{assets:()=>p,contentTitle:()=>s,default:()=>m,frontMatter:()=>o,metadata:()=>i,toc:()=>c});var a=n(87462),r=(n(67294),n(3905));const o={sidebar_position:1,title:"PSP37 Metadata"},s=void 0,i={unversionedId:"smart-contracts/PSP37/Extensions/metadata",id:"version-v4.0.0-beta/smart-contracts/PSP37/Extensions/metadata",title:"PSP37 Metadata",description:"This example shows how you can reuse the implementation of PSP37 token with PSP37Metadata extension.",source:"@site/versioned_docs/version-v4.0.0-beta/smart-contracts/PSP37/Extensions/metadata.md",sourceDirName:"smart-contracts/PSP37/Extensions",slug:"/smart-contracts/PSP37/Extensions/metadata",permalink:"/openbrush-contracts/smart-contracts/PSP37/Extensions/metadata",draft:!1,editUrl:"https://github.com/727-Ventures/openbrush-contracts/tree/main/docs/versioned_docs/version-v4.0.0-beta/smart-contracts/PSP37/Extensions/metadata.md",tags:[],version:"v4.0.0-beta",sidebarPosition:1,frontMatter:{sidebar_position:1,title:"PSP37 Metadata"},sidebar:"tutorialSidebar",previous:{title:"PSP37 Enumerable",permalink:"/openbrush-contracts/smart-contracts/PSP37/Extensions/enumerable"},next:{title:"PSP37 Batch",permalink:"/openbrush-contracts/smart-contracts/PSP37/Extensions/batch"}},p={},c=[{value:"Step 1: Implement PSP37Metadata",id:"step-1-implement-psp37metadata",level:2},{value:"Step 2: Define storage",id:"step-2-define-storage",level:2},{value:"Step 3: Define constructor",id:"step-3-define-constructor",level:2},{value:"Final code",id:"final-code",level:2}],l={toc:c},u="wrapper";function m(e){let{components:t,...n}=e;return(0,r.kt)(u,(0,a.Z)({},l,n,{components:t,mdxType:"MDXLayout"}),(0,r.kt)("p",null,"This example shows how you can reuse the implementation of ",(0,r.kt)("a",{parentName:"p",href:"https://github.com/Brushfam/openbrush-contracts/tree/main/contracts/token/psp37"},"PSP37")," token with ",(0,r.kt)("a",{parentName:"p",href:"https://github.com/Brushfam/openbrush-contracts/tree/main/contracts/token/psp37/extensions/metadata.rs"},"PSP37Metadata")," extension."),(0,r.kt)("p",null,"First, you should implement basic version of ",(0,r.kt)("a",{parentName:"p",href:"/openbrush-contracts/smart-contracts/PSP37/"},"PSP37"),"."),(0,r.kt)("h2",{id:"step-1-implement-psp37metadata"},"Step 1: Implement PSP37Metadata"),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-rust"},'#![cfg_attr(not(feature = "std"), no_std, no_main)]\n\n#[openbrush::implementation(PSP37, PSP37Metadata)]\n#[openbrush::contract]\npub mod my_psp37 {\n...\n')),(0,r.kt)("h2",{id:"step-2-define-storage"},"Step 2: Define storage"),(0,r.kt)("p",null,"Declare storage struct and declare the field related to the metadata module.\nThen you need to derive the ",(0,r.kt)("inlineCode",{parentName:"p"},"Storage")," trait and mark the corresponding field with\nthe ",(0,r.kt)("inlineCode",{parentName:"p"},"#[storage_field]")," attribute.\nDeriving this trait allows you to reuse the ",(0,r.kt)("inlineCode",{parentName:"p"},"PSP37Metadata")," extension in your\n",(0,r.kt)("inlineCode",{parentName:"p"},"PSP37")," implementation."),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-rust"},"#[derive(Default, Storage)]\n#[ink(storage)]\npub struct Contract {\n    ...\n    #[storage_field]\n    metadata: metadata::Data,\n}\n")),(0,r.kt)("h2",{id:"step-3-define-constructor"},"Step 3: Define constructor"),(0,r.kt)("p",null,"Define constructor. Your ",(0,r.kt)("inlineCode",{parentName:"p"},"PSP37Metadata")," contract is ready!"),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-rust"},"impl Contract {\n    #[ink(constructor)]\n    pub fn new() -> Self {\n        Self::default()\n    }\n}\n")),(0,r.kt)("h2",{id:"final-code"},"Final code"),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-rust"},'#![cfg_attr(not(feature = "std"), no_std, no_main)]\n\n#[openbrush::implementation(PSP37, PSP37Metadata)]\n#[openbrush::contract]\npub mod my_psp37 {\n    use openbrush::traits::{\n        Storage,\n        String,\n    };\n\n    #[derive(Default, Storage)]\n    #[ink(storage)]\n    pub struct Contract {\n        #[storage_field]\n        psp37: psp37::Data,\n        #[storage_field]\n        metadata: metadata::Data,\n    }\n\n    impl Contract {\n        #[ink(constructor)]\n        pub fn new() -> Self {\n            Self::default()\n        }\n\n        #[ink(message)]\n        pub fn set_attribute(&mut self, id: Id, key: String, data: String) -> Result<(), PSP37Error> {\n            metadata::Internal::_set_attribute(self, &id, &key, &data)\n        }\n    }\n}\n\n')),(0,r.kt)("p",null,"You can check an example of the usage of ",(0,r.kt)("a",{parentName:"p",href:"https://github.com/Brushfam/openbrush-contracts/tree/main/examples/psp37_extensions/metadata"},"PSP37 Metadata"),"."))}m.isMDXComponent=!0}}]);