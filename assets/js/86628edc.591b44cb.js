"use strict";(self.webpackChunkopenbrush=self.webpackChunkopenbrush||[]).push([[96204],{3905:(e,t,n)=>{n.d(t,{Zo:()=>u,kt:()=>f});var a=n(67294);function r(e,t,n){return t in e?Object.defineProperty(e,t,{value:n,enumerable:!0,configurable:!0,writable:!0}):e[t]=n,e}function i(e,t){var n=Object.keys(e);if(Object.getOwnPropertySymbols){var a=Object.getOwnPropertySymbols(e);t&&(a=a.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),n.push.apply(n,a)}return n}function s(e){for(var t=1;t<arguments.length;t++){var n=null!=arguments[t]?arguments[t]:{};t%2?i(Object(n),!0).forEach((function(t){r(e,t,n[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(n)):i(Object(n)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(n,t))}))}return e}function o(e,t){if(null==e)return{};var n,a,r=function(e,t){if(null==e)return{};var n,a,r={},i=Object.keys(e);for(a=0;a<i.length;a++)n=i[a],t.indexOf(n)>=0||(r[n]=e[n]);return r}(e,t);if(Object.getOwnPropertySymbols){var i=Object.getOwnPropertySymbols(e);for(a=0;a<i.length;a++)n=i[a],t.indexOf(n)>=0||Object.prototype.propertyIsEnumerable.call(e,n)&&(r[n]=e[n])}return r}var l=a.createContext({}),p=function(e){var t=a.useContext(l),n=t;return e&&(n="function"==typeof e?e(t):s(s({},t),e)),n},u=function(e){var t=p(e.components);return a.createElement(l.Provider,{value:t},e.children)},c="mdxType",d={inlineCode:"code",wrapper:function(e){var t=e.children;return a.createElement(a.Fragment,{},t)}},m=a.forwardRef((function(e,t){var n=e.components,r=e.mdxType,i=e.originalType,l=e.parentName,u=o(e,["components","mdxType","originalType","parentName"]),c=p(n),m=r,f=c["".concat(l,".").concat(m)]||c[m]||d[m]||i;return n?a.createElement(f,s(s({ref:t},u),{},{components:n})):a.createElement(f,s({ref:t},u))}));function f(e,t){var n=arguments,r=t&&t.mdxType;if("string"==typeof e||r){var i=n.length,s=new Array(i);s[0]=m;var o={};for(var l in t)hasOwnProperty.call(t,l)&&(o[l]=t[l]);o.originalType=e,o[c]="string"==typeof e?e:r,s[1]=o;for(var p=2;p<i;p++)s[p]=n[p];return a.createElement.apply(null,s)}return a.createElement.apply(null,n)}m.displayName="MDXCreateElement"},50104:(e,t,n)=>{n.r(t),n.d(t,{assets:()=>l,contentTitle:()=>s,default:()=>c,frontMatter:()=>i,metadata:()=>o,toc:()=>p});var a=n(87462),r=(n(67294),n(3905));const i={sidebar_position:5,title:"Pausable"},s=void 0,o={unversionedId:"smart-contracts/pausable",id:"version-1.1.0/smart-contracts/pausable",title:"Pausable",description:"This example shows how you can reuse the implementation of",source:"@site/versioned_docs/version-1.1.0/smart-contracts/pausable.md",sourceDirName:"smart-contracts",slug:"/smart-contracts/pausable",permalink:"/1.1.0/smart-contracts/pausable",draft:!1,editUrl:"https://github.com/727-Ventures/openbrush-contracts/tree/main/docs/versioned_docs/version-1.1.0/smart-contracts/pausable.md",tags:[],version:"1.1.0",sidebarPosition:5,frontMatter:{sidebar_position:5,title:"Pausable"},sidebar:"tutorialSidebar",previous:{title:"Reentrancy Guard",permalink:"/1.1.0/smart-contracts/reentrancy-guard"},next:{title:"Payment Splitter",permalink:"/1.1.0/smart-contracts/payment-splitter"}},l={},p=[{value:"Step 1: Include dependencies",id:"step-1-include-dependencies",level:2},{value:"Step 2: Add imports",id:"step-2-add-imports",level:2},{value:"Step 3: Define storage",id:"step-3-define-storage",level:2},{value:"Step 4: Inherit logic",id:"step-4-inherit-logic",level:2},{value:"Step 5: Define constructor",id:"step-5-define-constructor",level:2},{value:"Step 6: Customize your contract",id:"step-6-customize-your-contract",level:2}],u={toc:p};function c(e){let{components:t,...n}=e;return(0,r.kt)("wrapper",(0,a.Z)({},u,n,{components:t,mdxType:"MDXLayout"}),(0,r.kt)("p",null,"This example shows how you can reuse the implementation of\n",(0,r.kt)("a",{parentName:"p",href:"https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/security/pausable"},"pausable")," in ",(0,r.kt)("inlineCode",{parentName:"p"},"Flipper")," contract to ",(0,r.kt)("inlineCode",{parentName:"p"},"flip")," only if the contract is not paused."),(0,r.kt)("h2",{id:"step-1-include-dependencies"},"Step 1: Include dependencies"),(0,r.kt)("p",null,"Include dependencies to ",(0,r.kt)("inlineCode",{parentName:"p"},"pausable")," and ",(0,r.kt)("inlineCode",{parentName:"p"},"brush")," in the cargo file."),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-toml"},'[dependencies]\nink_primitives = { tag = "v3.0.0-rc6", git = "https://github.com/paritytech/ink", default-features = false }\nink_metadata = { tag = "v3.0.0-rc6", git = "https://github.com/paritytech/ink", default-features = false, features = ["derive"], optional = true }\nink_env = { tag = "v3.0.0-rc6", git = "https://github.com/paritytech/ink", default-features = false }\nink_storage = { tag = "v3.0.0-rc6", git = "https://github.com/paritytech/ink", default-features = false }\nink_lang = { tag = "v3.0.0-rc6", git = "https://github.com/paritytech/ink", default-features = false }\nink_prelude = { tag = "v3.0.0-rc6", git = "https://github.com/paritytech/ink", default-features = false }\n\nscale = { package = "parity-scale-codec", version = "2", default-features = false, features = ["derive"] }\nscale-info = { version = "1", default-features = false, features = ["derive"], optional = true }\n\n# These dependencies\npausable = { tag = "v1.0.0", git = "https://github.com/Supercolony-net/openbrush-contracts", default-features = false }\nbrush = { tag = "v1.0.0", git = "https://github.com/Supercolony-net/openbrush-contracts", default-features = false }\n\n[features]\ndefault = ["std"]\nstd = [\n   "ink_primitives/std",\n   "ink_metadata",\n   "ink_metadata/std",\n   "ink_env/std",\n   "ink_storage/std",\n   "ink_lang/std",\n   "scale/std",\n   "scale-info",\n   "scale-info/std",\n\n   # These dependencies\n   "pausable/std",\n   "brush/std",\n]\n')),(0,r.kt)("h2",{id:"step-2-add-imports"},"Step 2: Add imports"),(0,r.kt)("p",null,"Replace ",(0,r.kt)("inlineCode",{parentName:"p"},"ink::contract")," macro by ",(0,r.kt)("inlineCode",{parentName:"p"},"brush::contract"),".\nImport ",(0,r.kt)("strong",{parentName:"p"},"everything")," from ",(0,r.kt)("inlineCode",{parentName:"p"},"pausable::traits"),"."),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-rust"},"#[brush::contract]\npub mod my_pausable {\n   use pausable::traits::*;\n")),(0,r.kt)("h2",{id:"step-3-define-storage"},"Step 3: Define storage"),(0,r.kt)("p",null,"Declare storage struct and declare the field related to ",(0,r.kt)("inlineCode",{parentName:"p"},"PausableStorage"),".\nThen you need to derive ",(0,r.kt)("inlineCode",{parentName:"p"},"PausableStorage")," trait and mark corresponding field\nwith ",(0,r.kt)("inlineCode",{parentName:"p"},"#[PausableStorageField]")," attribute. Deriving this trait allows you to reuse\nthe default implementation of ",(0,r.kt)("inlineCode",{parentName:"p"},"Pausable"),"."),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-rust"},"#[ink(storage)]\n#[derive(Default, PausableStorage)]\npub struct MyFlipper {\n   #[PausableStorageField]\n   pause: PausableData,\n   flipped: bool,\n}\n")),(0,r.kt)("h2",{id:"step-4-inherit-logic"},"Step 4: Inherit logic"),(0,r.kt)("p",null,"Inherit the implementation of ",(0,r.kt)("inlineCode",{parentName:"p"},"Pausable"),". You can customize (override) methods in this ",(0,r.kt)("inlineCode",{parentName:"p"},"impl")," block."),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-rust"},"impl Pausable for MyFlipper {}\n")),(0,r.kt)("h2",{id:"step-5-define-constructor"},"Step 5: Define constructor"),(0,r.kt)("p",null,"Define constructor. Your basic version of ",(0,r.kt)("inlineCode",{parentName:"p"},"Pausable")," contract is ready!"),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-rust"},"impl MyFlipper {\n   #[ink(constructor)]\n   pub fn new() -> Self {\n      Self::default()\n   }\n}\n")),(0,r.kt)("h2",{id:"step-6-customize-your-contract"},"Step 6: Customize your contract"),(0,r.kt)("p",null,"Customize it by adding flipper logic. We will implement ",(0,r.kt)("inlineCode",{parentName:"p"},"flip")," method marked with ",(0,r.kt)("inlineCode",{parentName:"p"},"when_not_paused")," modifier."),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-rust"},"impl MyFlipper {\n   #[ink(constructor)]\n   pub fn new() -> Self {\n      Self::default()\n   }\n\n   #[ink(message)]\n   #[brush::modifiers(when_not_paused)]\n   pub fn flip(&mut self) {\n      self.flipped = !self.flipped;\n   }\n\n   #[ink(message)]\n   pub fn pause(&mut self) {\n      self._pause()\n   }\n\n   #[ink(message)]\n   pub fn unpause(&mut self) {\n      self._unpause()\n   }\n}\n\nimpl Pausable for MyFlipper {}\n")))}c.isMDXComponent=!0}}]);