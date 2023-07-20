"use strict";(self.webpackChunkopenbrush=self.webpackChunkopenbrush||[]).push([[867],{3905:(e,t,a)=>{a.d(t,{Zo:()=>p,kt:()=>m});var r=a(67294);function n(e,t,a){return t in e?Object.defineProperty(e,t,{value:a,enumerable:!0,configurable:!0,writable:!0}):e[t]=a,e}function o(e,t){var a=Object.keys(e);if(Object.getOwnPropertySymbols){var r=Object.getOwnPropertySymbols(e);t&&(r=r.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),a.push.apply(a,r)}return a}function s(e){for(var t=1;t<arguments.length;t++){var a=null!=arguments[t]?arguments[t]:{};t%2?o(Object(a),!0).forEach((function(t){n(e,t,a[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(a)):o(Object(a)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(a,t))}))}return e}function l(e,t){if(null==e)return{};var a,r,n=function(e,t){if(null==e)return{};var a,r,n={},o=Object.keys(e);for(r=0;r<o.length;r++)a=o[r],t.indexOf(a)>=0||(n[a]=e[a]);return n}(e,t);if(Object.getOwnPropertySymbols){var o=Object.getOwnPropertySymbols(e);for(r=0;r<o.length;r++)a=o[r],t.indexOf(a)>=0||Object.prototype.propertyIsEnumerable.call(e,a)&&(n[a]=e[a])}return n}var i=r.createContext({}),c=function(e){var t=r.useContext(i),a=t;return e&&(a="function"==typeof e?e(t):s(s({},t),e)),a},p=function(e){var t=c(e.components);return r.createElement(i.Provider,{value:t},e.children)},h="mdxType",d={inlineCode:"code",wrapper:function(e){var t=e.children;return r.createElement(r.Fragment,{},t)}},u=r.forwardRef((function(e,t){var a=e.components,n=e.mdxType,o=e.originalType,i=e.parentName,p=l(e,["components","mdxType","originalType","parentName"]),h=c(a),u=n,m=h["".concat(i,".").concat(u)]||h[u]||d[u]||o;return a?r.createElement(m,s(s({ref:t},p),{},{components:a})):r.createElement(m,s({ref:t},p))}));function m(e,t){var a=arguments,n=t&&t.mdxType;if("string"==typeof e||n){var o=a.length,s=new Array(o);s[0]=u;var l={};for(var i in t)hasOwnProperty.call(t,i)&&(l[i]=t[i]);l.originalType=e,l[h]="string"==typeof e?e:n,s[1]=l;for(var c=2;c<o;c++)s[c]=a[c];return r.createElement.apply(null,s)}return r.createElement.apply(null,a)}u.displayName="MDXCreateElement"},57679:(e,t,a)=>{a.r(t),a.d(t,{assets:()=>i,contentTitle:()=>s,default:()=>d,frontMatter:()=>o,metadata:()=>l,toc:()=>c});var r=a(87462),n=(a(67294),a(3905));const o={sidebar_position:1,title:"Overview"},s=void 0,l={unversionedId:"smart-contracts/example/overview",id:"version-3.1.1/smart-contracts/example/overview",title:"Overview",description:"This example will show you how you can reuse OpenBrush smart contracts and macros in your",source:"@site/versioned_docs/version-3.1.1/smart-contracts/example/overview.md",sourceDirName:"smart-contracts/example",slug:"/smart-contracts/example/overview",permalink:"/openbrush-contracts/3.1.1/smart-contracts/example/overview",draft:!1,editUrl:"https://github.com/727-Ventures/openbrush-contracts/tree/main/docs/versioned_docs/version-3.1.1/smart-contracts/example/overview.md",tags:[],version:"3.1.1",sidebarPosition:1,frontMatter:{sidebar_position:1,title:"Overview"},sidebar:"tutorialSidebar",previous:{title:"PSP37 Burnable",permalink:"/openbrush-contracts/3.1.1/smart-contracts/PSP37/Extensions/burnable"},next:{title:"Setup the project",permalink:"/openbrush-contracts/3.1.1/smart-contracts/example/setup_project"}},i={},c=[{value:"Lending of assets accepted by the smart contract",id:"lending-of-assets-accepted-by-the-smart-contract",level:2},{value:"Borrowing of assets by depositing accepted assets as collateral",id:"borrowing-of-assets-by-depositing-accepted-assets-as-collateral",level:2},{value:"Repaying the loan",id:"repaying-the-loan",level:2},{value:"Withdraw deposited assets",id:"withdraw-deposited-assets",level:2},{value:"Liquidate a loan",id:"liquidate-a-loan",level:2},{value:"Allow an asset for lending or being used as a collateral",id:"allow-an-asset-for-lending-or-being-used-as-a-collateral",level:2},{value:"Pause the contract",id:"pause-the-contract",level:2}],p={toc:c},h="wrapper";function d(e){let{components:t,...a}=e;return(0,n.kt)(h,(0,r.Z)({},p,a,{components:t,mdxType:"MDXLayout"}),(0,n.kt)("p",null,"This example will show you how you can reuse OpenBrush smart contracts and macros in your\nproject to ease the development process. We will also pay attention to the project\nstructure to keep the maintenance and future development of the project simple."),(0,n.kt)("p",null,"We will be implementing a simple lending protocol, in which users can lend\n",(0,n.kt)("a",{parentName:"p",href:"/smart-contracts/PSP22"},"PSP22")," tokens, borrow them against a collateral token,\nrepay their loans with interest, and of course withdraw the deposited assets.\nWe will create a ",(0,n.kt)("a",{parentName:"p",href:"/smart-contracts/PSP22"},"PSP22")," implementation which will be used\nfor a stable coin and a collateral token, another ",(0,n.kt)("a",{parentName:"p",href:"/smart-contracts/PSP22"},"PSP22"),"\ntoken which will represent the shares of assets in the contract,\n",(0,n.kt)("a",{parentName:"p",href:"/smart-contracts/PSP34"},"PSP34")," token which will represent the loans and the\nlending contract itself. The simple ",(0,n.kt)("a",{parentName:"p",href:"/smart-contracts/PSP22"},"PSP22")," token\nimplementation will be created just for this example and to test the contract's functions.\nThe contract will have the following features:"),(0,n.kt)("h2",{id:"lending-of-assets-accepted-by-the-smart-contract"},"Lending of assets accepted by the smart contract"),(0,n.kt)("p",null,"Users can lend ",(0,n.kt)("a",{parentName:"p",href:"/smart-contracts/PSP22"},"PSP22")," tokens, which are accepted by the\ncontract. The allowance of lending specific tokens is decided in the smart contract\nby the accounts which have the Manager role. Upon lending the user gets a\n",(0,n.kt)("a",{parentName:"p",href:"/smart-contracts/PSP22"},"PSP22")," token representing their share of the asset pool."),(0,n.kt)("h2",{id:"borrowing-of-assets-by-depositing-accepted-assets-as-collateral"},"Borrowing of assets by depositing accepted assets as collateral"),(0,n.kt)("p",null,"Users can borrow ",(0,n.kt)("a",{parentName:"p",href:"/smart-contracts/PSP22"},"PSP22")," tokens, which are available in\nthe contract. To borrow an asset, the user has to deposit an accepted\n",(0,n.kt)("a",{parentName:"p",href:"/smart-contracts/PSP22"},"PSP22")," token as collateral. The allowance of specific\ntokens being used as collateral is decided in the smart contract by the accounts\nwhich have the Manager role. The value of the borrowed assets can be equal at most\nto 70% of the value of the deposited collateral. If the value of the deposited\ncollateral drops to or below 75% of the original value, the loan can be liquidated.\nUpon borrowing the assets user gets a ",(0,n.kt)("a",{parentName:"p",href:"/smart-contracts/PSP34"},"PSP34")," token\nrepresenting info about their loan (how much assets were borrowed, when did they\nborrow, what asset was borrowed, what asset was used as collateral, amount of\ncollateral asset deposited, the liquidation price of the loan and if it was liquidated\nor not). This NFT token can be then used to repay the loan and get the collateral back."),(0,n.kt)("h2",{id:"repaying-the-loan"},"Repaying the loan"),(0,n.kt)("p",null,"Users can repay their loan by depositing the borrowed amount of the borrowed assets\nwith the interest which is calculated by the contract. Our contract has an interest\nrate of 10% per year. Users can repay the whole loan or a portion of the loan. The\nuser will use their NFT to repay the loan. If the loan was liquidated in the meantime,\nthey do not get their collateral back and the NFT is burned."),(0,n.kt)("h2",{id:"withdraw-deposited-assets"},"Withdraw deposited assets"),(0,n.kt)("p",null,"Users will deposit their share tokens to the smart contract and get back the deposited\nassets along with the interest generated if any."),(0,n.kt)("h2",{id:"liquidate-a-loan"},"Liquidate a loan"),(0,n.kt)("p",null,"Users can liquidate a loan which's collateral value is below or equal to 75% of the\noriginal value of the collateral. After the loan is liquidated, the liquidator\ngets 1% of the liquidated assets. "),(0,n.kt)("h2",{id:"allow-an-asset-for-lending-or-being-used-as-a-collateral"},"Allow an asset for lending or being used as a collateral"),(0,n.kt)("p",null,"Users with the Manager role can allow an asset to be available for lending and\nborrowing or for being used as collateral."),(0,n.kt)("h2",{id:"pause-the-contract"},"Pause the contract"),(0,n.kt)("p",null,"Users with the Manager role can pause the contract. When the contract is paused,\nusers can not deposit new assets for lending or borrowing assets. Users can still\nrepay their loans, liquidate loans or withdraw their deposits when paused."))}d.isMDXComponent=!0}}]);