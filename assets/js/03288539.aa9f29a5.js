"use strict";(self.webpackChunkopenbrush=self.webpackChunkopenbrush||[]).push([[21295],{3905:(e,t,n)=>{n.d(t,{Zo:()=>m,kt:()=>f});var r=n(67294);function o(e,t,n){return t in e?Object.defineProperty(e,t,{value:n,enumerable:!0,configurable:!0,writable:!0}):e[t]=n,e}function a(e,t){var n=Object.keys(e);if(Object.getOwnPropertySymbols){var r=Object.getOwnPropertySymbols(e);t&&(r=r.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),n.push.apply(n,r)}return n}function c(e){for(var t=1;t<arguments.length;t++){var n=null!=arguments[t]?arguments[t]:{};t%2?a(Object(n),!0).forEach((function(t){o(e,t,n[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(n)):a(Object(n)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(n,t))}))}return e}function l(e,t){if(null==e)return{};var n,r,o=function(e,t){if(null==e)return{};var n,r,o={},a=Object.keys(e);for(r=0;r<a.length;r++)n=a[r],t.indexOf(n)>=0||(o[n]=e[n]);return o}(e,t);if(Object.getOwnPropertySymbols){var a=Object.getOwnPropertySymbols(e);for(r=0;r<a.length;r++)n=a[r],t.indexOf(n)>=0||Object.prototype.propertyIsEnumerable.call(e,n)&&(o[n]=e[n])}return o}var i=r.createContext({}),s=function(e){var t=r.useContext(i),n=t;return e&&(n="function"==typeof e?e(t):c(c({},t),e)),n},m=function(e){var t=s(e.components);return r.createElement(i.Provider,{value:t},e.children)},p="mdxType",u={inlineCode:"code",wrapper:function(e){var t=e.children;return r.createElement(r.Fragment,{},t)}},d=r.forwardRef((function(e,t){var n=e.components,o=e.mdxType,a=e.originalType,i=e.parentName,m=l(e,["components","mdxType","originalType","parentName"]),p=s(n),d=o,f=p["".concat(i,".").concat(d)]||p[d]||u[d]||a;return n?r.createElement(f,c(c({ref:t},m),{},{components:n})):r.createElement(f,c({ref:t},m))}));function f(e,t){var n=arguments,o=t&&t.mdxType;if("string"==typeof e||o){var a=n.length,c=new Array(a);c[0]=d;var l={};for(var i in t)hasOwnProperty.call(t,i)&&(l[i]=t[i]);l.originalType=e,l[p]="string"==typeof e?e:o,c[1]=l;for(var s=2;s<a;s++)c[s]=n[s];return r.createElement.apply(null,c)}return r.createElement.apply(null,n)}d.displayName="MDXCreateElement"},33546:(e,t,n)=>{n.r(t),n.d(t,{assets:()=>i,contentTitle:()=>c,default:()=>u,frontMatter:()=>a,metadata:()=>l,toc:()=>s});var r=n(87462),o=(n(67294),n(3905));const a={sidebar_position:8,title:"Timelock Controller"},c=void 0,l={unversionedId:"smart-contracts/timelock-controller",id:"version-v4.0.0-beta/smart-contracts/timelock-controller",title:"Timelock Controller",description:"This example shows how you can reuse the implementation of",source:"@site/versioned_docs/version-v4.0.0-beta/smart-contracts/timelock-controller.md",sourceDirName:"smart-contracts",slug:"/smart-contracts/timelock-controller",permalink:"/openbrush-contracts/smart-contracts/timelock-controller",draft:!1,editUrl:"https://github.com/727-Ventures/openbrush-contracts/tree/main/docs/versioned_docs/version-v4.0.0-beta/smart-contracts/timelock-controller.md",tags:[],version:"v4.0.0-beta",sidebarPosition:8,frontMatter:{sidebar_position:8,title:"Timelock Controller"},sidebar:"tutorialSidebar",previous:{title:"PSP22 Token Timelock",permalink:"/openbrush-contracts/smart-contracts/PSP22/Utils/token-timelock"},next:{title:"PSP22 Pallet",permalink:"/openbrush-contracts/smart-contracts/PSP22-Pallet/"}},i={},s=[{value:"Step 1: Import default implementation",id:"step-1-import-default-implementation",level:2},{value:"Step 2: Define constructor",id:"step-2-define-constructor",level:2},{value:"Final code",id:"final-code",level:2}],m={toc:s},p="wrapper";function u(e){let{components:t,...n}=e;return(0,o.kt)(p,(0,r.Z)({},m,n,{components:t,mdxType:"MDXLayout"}),(0,o.kt)("p",null,"This example shows how you can reuse the implementation of\n",(0,o.kt)("a",{parentName:"p",href:"https://github.com/Brushfam/openbrush-contracts/tree/main/contracts/src/governance/timelock_controller"},"timelock-controller"),"."),(0,o.kt)("h2",{id:"step-1-import-default-implementation"},"Step 1: Import default implementation"),(0,o.kt)("p",null,"With ",(0,o.kt)("a",{parentName:"p",href:"/openbrush-contracts/smart-contracts/overview/#the-default-toml-of-your-project-with-openbrush"},"default ",(0,o.kt)("inlineCode",{parentName:"a"},"Cargo.toml")),",\nyou need to enable corresponding features, embed modules data structures and implement them via ",(0,o.kt)("inlineCode",{parentName:"p"},"#[openbrush::implementation]")," macro\nas described in ",(0,o.kt)("a",{parentName:"p",href:"/openbrush-contracts/smart-contracts/overview/#reuse-implementation-of-traits-from-openbrush"},"that section"),"."),(0,o.kt)("p",null,"The main traits are ",(0,o.kt)("inlineCode",{parentName:"p"},"AccessControl")," and ",(0,o.kt)("inlineCode",{parentName:"p"},"TimelockController"),"."),(0,o.kt)("h2",{id:"step-2-define-constructor"},"Step 2: Define constructor"),(0,o.kt)("p",null,"Define constructor where you init admin of the contract."),(0,o.kt)("pre",null,(0,o.kt)("code",{parentName:"pre",className:"language-rust"},"impl Contract {\n    #[ink(constructor)]\n    pub fn new(min_delay: Timestamp, proposers: Vec<AccountId>, executors: Vec<AccountId>) -> Self {\n        let mut instance = Self::default();\n\n        let caller = Self::env().caller();\n        // `TimelockController` and `AccessControl` have `_init_with_admin` methods.\n        // You need to call it for each trait separately, to initialize everything for these traits.\n        access_control::Internal::_init_with_admin(instance, caller);\n        timelock_controller::Internal::_init_with_admin(instance, caller, min_delay, proposers, executors);\n        \n        instance\n    }\n}\n")),(0,o.kt)("h2",{id:"final-code"},"Final code"),(0,o.kt)("pre",null,(0,o.kt)("code",{parentName:"pre",className:"language-rust"},'#![cfg_attr(not(feature = "std"), no_std, no_main)]\n\n#[openbrush::implementation(AccessControl, TimelockController)]\n#[openbrush::contract]\npub mod my_timelock_controller {\n    use ink::prelude::vec::Vec;\n    use openbrush::traits::Storage;\n\n    #[ink(storage)]\n    #[derive(Default, Storage)]\n    pub struct Contract {\n        #[storage_field]\n        access_control: access_control::Data,\n        #[storage_field]\n        timelock: timelock_controller::Data,\n    }\n\n    impl Contract {\n        #[ink(constructor)]\n        pub fn new(min_delay: Timestamp, proposers: Vec<AccountId>, executors: Vec<AccountId>) -> Self {\n            let mut instance = Self::default();\n\n            let caller = Self::env().caller();\n            // `TimelockController` and `AccessControl` have `_init_with_admin` methods.\n            // You need to call it for each trait separately, to initialize everything for these traits.\n            access_control::Internal::_init_with_admin(&mut instance, Some(caller));\n            timelock_controller::Internal::_init_with_admin(\n                &mut instance,\n                Some(caller),\n                min_delay,\n                proposers,\n                executors,\n            );\n\n            instance\n        }\n    }\n}\n\n')),(0,o.kt)("p",null,"You can check an example of the usage of ",(0,o.kt)("a",{parentName:"p",href:"https://github.com/Brushfam/openbrush-contracts/tree/main/examples/timelock_controller"},"TimelockController"),"."))}u.isMDXComponent=!0}}]);