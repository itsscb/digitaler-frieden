let _=`string`,S=`utf-8`,W=1,U=0,$=`Object`,Z=`boolean`,O=128,R=`undefined`,Q=null,Y=`number`,a1=4,X=`function`,N=Array,T=Error,a0=FinalizationRegistry,a2=Object,V=Uint8Array,P=undefined;var c=(a=>b[a]);var M=(async(b)=>{if(a!==P)return a;if(typeof b===R){b=new URL(`digitaler-frieden_bg.wasm`,import.meta.url)};const c=I();if(typeof b===_||typeof Request===X&&b instanceof Request||typeof URL===X&&b instanceof URL){b=fetch(b)};J(c);const {instance:d,module:e}=await H(await b,c);return K(d,e)});var K=((b,c)=>{a=b.exports;M.__wbindgen_wasm_module=c;s=Q;q=Q;E=Q;h=Q;a.__wbindgen_start();return a});var H=(async(a,b)=>{if(typeof Response===X&&a instanceof Response){if(typeof WebAssembly.instantiateStreaming===X){try{return await WebAssembly.instantiateStreaming(a,b)}catch(b){if(a.headers.get(`Content-Type`)!=`application/wasm`){console.warn(`\`WebAssembly.instantiateStreaming\` failed because your server does not serve wasm with \`application/wasm\` MIME type. Falling back to \`WebAssembly.instantiate\` which is slower. Original error:\\n`,b)}else{throw b}}};const c=await a.arrayBuffer();return await WebAssembly.instantiate(c,b)}else{const c=await WebAssembly.instantiate(a,b);if(c instanceof WebAssembly.Instance){return {instance:c,module:a}}else{return c}}});var G=((a,b)=>{a=a>>>U;const c=F();const d=c.subarray(a/a1,a/a1+ b);const e=[];for(let a=U;a<d.length;a++){e.push(f(d[a]))};return e});var B=((b,c,d)=>{a._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__he3482e606ec2d010(b,c,k(d))});var r=(()=>{if(q===Q||q.byteLength===U){q=new Int32Array(a.memory.buffer)};return q});var F=(()=>{if(E===Q||E.byteLength===U){E=new Uint32Array(a.memory.buffer)};return E});var i=(()=>{if(h===Q||h.byteLength===U){h=new V(a.memory.buffer)};return h});var C=((c,d,e)=>{try{a._dyn_core__ops__function__FnMut___A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h047b4422b1752603(c,d,y(e))}finally{b[x++]=P}});var A=((b,c,d,e)=>{const f={a:b,b:c,cnt:W,dtor:d};const g=(...b)=>{f.cnt++;const c=f.a;f.a=U;try{return e(c,f.b,...b)}finally{if(--f.cnt===U){a.__wbindgen_export_2.get(f.dtor)(c,f.b);v.unregister(f)}else{f.a=c}}};g.original=f;v.register(g,f,f);return g});var u=(a=>{const b=typeof a;if(b==Y||b==Z||a==Q){return `${a}`};if(b==_){return `"${a}"`};if(b==`symbol`){const b=a.description;if(b==Q){return `Symbol`}else{return `Symbol(${b})`}};if(b==X){const b=a.name;if(typeof b==_&&b.length>U){return `Function(${b})`}else{return `Function`}};if(N.isArray(a)){const b=a.length;let c=`[`;if(b>U){c+=u(a[U])};for(let d=W;d<b;d++){c+=`, `+ u(a[d])};c+=`]`;return c};const c=/\[object ([^\]]+)\]/.exec(toString.call(a));let d;if(c.length>W){d=c[W]}else{return toString.call(a)};if(d==$){try{return `Object(`+ JSON.stringify(a)+ `)`}catch(a){return $}};if(a instanceof T){return `${a.name}: ${a.message}\n${a.stack}`};return d});var o=((a,b,c)=>{if(c===P){const c=m.encode(a);const d=b(c.length,W)>>>U;i().subarray(d,d+ c.length).set(c);l=c.length;return d};let d=a.length;let e=b(d,W)>>>U;const f=i();let g=U;for(;g<d;g++){const b=a.charCodeAt(g);if(b>127)break;f[e+ g]=b};if(g!==d){if(g!==U){a=a.slice(g)};e=c(e,d,d=g+ a.length*3,W)>>>U;const b=i().subarray(e+ g,e+ d);const f=n(a,b);g+=f.written;e=c(e,d,g,W)>>>U};l=g;return e});var e=(a=>{if(a<132)return;b[a]=d;d=a});var k=(a=>{if(d===b.length)b.push(b.length+ W);const c=d;d=b[c];b[c]=a;return c});var j=((a,b)=>{a=a>>>U;return g.decode(i().subarray(a,a+ b))});var f=(a=>{const b=c(a);e(a);return b});var z=((c,d,e)=>{try{a._dyn_core__ops__function__Fn___A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h82d851e3aa54ebfb(c,d,y(e))}finally{b[x++]=P}});var y=(a=>{if(x==W)throw new T(`out of js stack`);b[--x]=a;return x});var L=(b=>{if(a!==P)return a;const c=I();J(c);if(!(b instanceof WebAssembly.Module)){b=new WebAssembly.Module(b)};const d=new WebAssembly.Instance(b,c);return K(d,b)});var w=((b,c,d,e)=>{const f={a:b,b:c,cnt:W,dtor:d};const g=(...b)=>{f.cnt++;try{return e(f.a,f.b,...b)}finally{if(--f.cnt===U){a.__wbindgen_export_2.get(f.dtor)(f.a,f.b);f.a=U;v.unregister(f)}}};g.original=f;v.register(g,f,f);return g});var p=(a=>a===P||a===Q);var J=((a,b)=>{});var t=(()=>{if(s===Q||s.byteLength===U){s=new Float64Array(a.memory.buffer)};return s});var I=(()=>{const b={};b.wbg={};b.wbg.__wbindgen_object_drop_ref=(a=>{f(a)});b.wbg.__wbindgen_string_new=((a,b)=>{const c=j(a,b);return k(c)});b.wbg.__wbindgen_object_clone_ref=(a=>{const b=c(a);return k(b)});b.wbg.__wbg_cachekey_b81c1aacc6a0645c=((a,b)=>{const d=c(b).__yew_subtree_cache_key;r()[a/a1+ W]=p(d)?U:d;r()[a/a1+ U]=!p(d)});b.wbg.__wbg_subtreeid_e80a1798fee782f9=((a,b)=>{const d=c(b).__yew_subtree_id;r()[a/a1+ W]=p(d)?U:d;r()[a/a1+ U]=!p(d)});b.wbg.__wbg_setsubtreeid_e1fab6b578c800cf=((a,b)=>{c(a).__yew_subtree_id=b>>>U});b.wbg.__wbg_setcachekey_75bcd45312087529=((a,b)=>{c(a).__yew_subtree_cache_key=b>>>U});b.wbg.__wbindgen_jsval_eq=((a,b)=>{const d=c(a)===c(b);return d});b.wbg.__wbindgen_cb_drop=(a=>{const b=f(a).original;if(b.cnt--==W){b.a=U;return !0};const c=!1;return c});b.wbg.__wbg_setlistenerid_f2e783343fa0cec1=((a,b)=>{c(a).__yew_listener_id=b>>>U});b.wbg.__wbg_listenerid_6dcf1c62b7b7de58=((a,b)=>{const d=c(b).__yew_listener_id;r()[a/a1+ W]=p(d)?U:d;r()[a/a1+ U]=!p(d)});b.wbg.__wbg_new_abda76e883ba8a5f=(()=>{const a=new T();return k(a)});b.wbg.__wbg_stack_658279fe44541cf6=((b,d)=>{const e=c(d).stack;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/a1+ W]=g;r()[b/a1+ U]=f});b.wbg.__wbg_error_f851667af71bcfc6=((b,c)=>{let d;let e;try{d=b;e=c;console.error(j(b,c))}finally{a.__wbindgen_free(d,e,W)}});b.wbg.__wbg_queueMicrotask_481971b0d87f3dd4=(a=>{queueMicrotask(c(a))});b.wbg.__wbg_queueMicrotask_3cbae2ec6b6cd3d6=(a=>{const b=c(a).queueMicrotask;return k(b)});b.wbg.__wbindgen_is_function=(a=>{const b=typeof c(a)===X;return b});b.wbg.__wbindgen_string_get=((b,d)=>{const e=c(d);const f=typeof e===_?e:P;var g=p(f)?U:o(f,a.__wbindgen_malloc,a.__wbindgen_realloc);var h=l;r()[b/a1+ W]=h;r()[b/a1+ U]=g});b.wbg.__wbindgen_is_string=(a=>{const b=typeof c(a)===_;return b});b.wbg.__wbindgen_is_object=(a=>{const b=c(a);const d=typeof b===`object`&&b!==Q;return d});b.wbg.__wbindgen_is_undefined=(a=>{const b=c(a)===P;return b});b.wbg.__wbindgen_in=((a,b)=>{const d=c(a) in c(b);return d});b.wbg.__wbindgen_error_new=((a,b)=>{const c=new T(j(a,b));return k(c)});b.wbg.__wbg_crypto_566d7465cdbb6b7a=(a=>{const b=c(a).crypto;return k(b)});b.wbg.__wbg_process_dc09a8c7d59982f6=(a=>{const b=c(a).process;return k(b)});b.wbg.__wbg_versions_d98c6400c6ca2bd8=(a=>{const b=c(a).versions;return k(b)});b.wbg.__wbg_node_caaf83d002149bd5=(a=>{const b=c(a).node;return k(b)});b.wbg.__wbg_msCrypto_0b84745e9245cdf6=(a=>{const b=c(a).msCrypto;return k(b)});b.wbg.__wbg_require_94a9da52636aacbf=function(){return D((()=>{const a=module.require;return k(a)}),arguments)};b.wbg.__wbg_randomFillSync_290977693942bf03=function(){return D(((a,b)=>{c(a).randomFillSync(f(b))}),arguments)};b.wbg.__wbg_getRandomValues_260cc23a41afad9a=function(){return D(((a,b)=>{c(a).getRandomValues(c(b))}),arguments)};b.wbg.__wbindgen_jsval_loose_eq=((a,b)=>{const d=c(a)==c(b);return d});b.wbg.__wbindgen_boolean_get=(a=>{const b=c(a);const d=typeof b===Z?(b?W:U):2;return d});b.wbg.__wbindgen_number_get=((a,b)=>{const d=c(b);const e=typeof d===Y?d:P;t()[a/8+ W]=p(e)?U:e;r()[a/a1+ U]=!p(e)});b.wbg.__wbindgen_as_number=(a=>{const b=+c(a);return b});b.wbg.__wbindgen_number_new=(a=>{const b=a;return k(b)});b.wbg.__wbg_getwithrefkey_edc2c8960f0f1191=((a,b)=>{const d=c(a)[c(b)];return k(d)});b.wbg.__wbg_set_f975102236d3c502=((a,b,d)=>{c(a)[f(b)]=f(d)});b.wbg.__wbg_error_a526fb08a0205972=((b,c)=>{var d=G(b,c).slice();a.__wbindgen_free(b,c*a1,a1);console.error(...d)});b.wbg.__wbg_log_7c3433e130418e14=((b,c)=>{var d=G(b,c).slice();a.__wbindgen_free(b,c*a1,a1);console.log(...d)});b.wbg.__wbg_body_edb1908d3ceff3a1=(a=>{const b=c(a).body;return p(b)?U:k(b)});b.wbg.__wbg_createElement_8bae7856a4bb7411=function(){return D(((a,b,d)=>{const e=c(a).createElement(j(b,d));return k(e)}),arguments)};b.wbg.__wbg_createElementNS_556a62fb298be5a2=function(){return D(((a,b,d,e,f)=>{const g=c(a).createElementNS(b===U?P:j(b,d),j(e,f));return k(g)}),arguments)};b.wbg.__wbg_createTextNode_0c38fd80a5b2284d=((a,b,d)=>{const e=c(a).createTextNode(j(b,d));return k(e)});b.wbg.__wbg_querySelector_a5f74efc5fa193dd=function(){return D(((a,b,d)=>{const e=c(a).querySelector(j(b,d));return p(e)?U:k(e)}),arguments)};b.wbg.__wbg_instanceof_Window_f401953a2cf86220=(a=>{let b;try{b=c(a) instanceof Window}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_document_5100775d18896c16=(a=>{const b=c(a).document;return p(b)?U:k(b)});b.wbg.__wbg_location_2951b5ee34f19221=(a=>{const b=c(a).location;return k(b)});b.wbg.__wbg_history_bc4057de66a2015f=function(){return D((a=>{const b=c(a).history;return k(b)}),arguments)};b.wbg.__wbg_instanceof_Element_6945fc210db80ea9=(a=>{let b;try{b=c(a) instanceof Element}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_namespaceURI_5235ee79fd5f6781=((b,d)=>{const e=c(d).namespaceURI;var f=p(e)?U:o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);var g=l;r()[b/a1+ W]=g;r()[b/a1+ U]=f});b.wbg.__wbg_setinnerHTML_26d69b59e1af99c7=((a,b,d)=>{c(a).innerHTML=j(b,d)});b.wbg.__wbg_outerHTML_e073aa84e7bc1eaf=((b,d)=>{const e=c(d).outerHTML;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/a1+ W]=g;r()[b/a1+ U]=f});b.wbg.__wbg_removeAttribute_1b10a06ae98ebbd1=function(){return D(((a,b,d)=>{c(a).removeAttribute(j(b,d))}),arguments)};b.wbg.__wbg_setAttribute_3c9f6c303b696daa=function(){return D(((a,b,d,e,f)=>{c(a).setAttribute(j(b,d),j(e,f))}),arguments)};b.wbg.__wbg_href_2edbae9e92cdfeff=((b,d)=>{const e=c(d).href;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/a1+ W]=g;r()[b/a1+ U]=f});b.wbg.__wbg_parentNode_6be3abff20e1a5fb=(a=>{const b=c(a).parentNode;return p(b)?U:k(b)});b.wbg.__wbg_parentElement_347524db59fc2976=(a=>{const b=c(a).parentElement;return p(b)?U:k(b)});b.wbg.__wbg_childNodes_118168e8b23bcb9b=(a=>{const b=c(a).childNodes;return k(b)});b.wbg.__wbg_lastChild_83efe6d5da370e1f=(a=>{const b=c(a).lastChild;return p(b)?U:k(b)});b.wbg.__wbg_nextSibling_709614fdb0fb7a66=(a=>{const b=c(a).nextSibling;return p(b)?U:k(b)});b.wbg.__wbg_setnodeValue_94b86af0cda24b90=((a,b,d)=>{c(a).nodeValue=b===U?P:j(b,d)});b.wbg.__wbg_textContent_8e392d624539e731=((b,d)=>{const e=c(d).textContent;var f=p(e)?U:o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);var g=l;r()[b/a1+ W]=g;r()[b/a1+ U]=f});b.wbg.__wbg_cloneNode_e19c313ea20d5d1d=function(){return D((a=>{const b=c(a).cloneNode();return k(b)}),arguments)};b.wbg.__wbg_insertBefore_d2a001abf538c1f8=function(){return D(((a,b,d)=>{const e=c(a).insertBefore(c(b),c(d));return k(e)}),arguments)};b.wbg.__wbg_removeChild_96bbfefd2f5a0261=function(){return D(((a,b)=>{const d=c(a).removeChild(c(b));return k(d)}),arguments)};b.wbg.__wbg_addEventListener_4283b15b4f039eb5=function(){return D(((a,b,d,e,f)=>{c(a).addEventListener(j(b,d),c(e),c(f))}),arguments)};b.wbg.__wbg_removeEventListener_5d31483804421bfa=function(){return D(((a,b,d,e,f)=>{c(a).removeEventListener(j(b,d),c(e),f!==U)}),arguments)};b.wbg.__wbg_setchecked_931ff2ed2cd3ebfd=((a,b)=>{c(a).checked=b!==U});b.wbg.__wbg_value_47fe6384562f52ab=((b,d)=>{const e=c(d).value;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/a1+ W]=g;r()[b/a1+ U]=f});b.wbg.__wbg_setvalue_78cb4f1fef58ae98=((a,b,d)=>{c(a).value=j(b,d)});b.wbg.__wbg_ctrlKey_008695ce60a588f5=(a=>{const b=c(a).ctrlKey;return b});b.wbg.__wbg_shiftKey_1e76dbfcdd36a4b4=(a=>{const b=c(a).shiftKey;return b});b.wbg.__wbg_altKey_07da841b54bd3ed6=(a=>{const b=c(a).altKey;return b});b.wbg.__wbg_metaKey_86bfd3b0d3a8083f=(a=>{const b=c(a).metaKey;return b});b.wbg.__wbg_bubbles_abce839854481bc6=(a=>{const b=c(a).bubbles;return b});b.wbg.__wbg_cancelBubble_c0aa3172524eb03c=(a=>{const b=c(a).cancelBubble;return b});b.wbg.__wbg_composedPath_58473fd5ae55f2cd=(a=>{const b=c(a).composedPath();return k(b)});b.wbg.__wbg_preventDefault_b1a4aafc79409429=(a=>{c(a).preventDefault()});b.wbg.__wbg_state_9cc3f933b7d50acb=function(){return D((a=>{const b=c(a).state;return k(b)}),arguments)};b.wbg.__wbg_pushState_b8e8d346f8bb33fd=function(){return D(((a,b,d,e,f,g)=>{c(a).pushState(c(b),j(d,e),f===U?P:j(f,g))}),arguments)};b.wbg.__wbg_instanceof_ShadowRoot_9db040264422e84a=(a=>{let b;try{b=c(a) instanceof ShadowRoot}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_host_c667c7623404d6bf=(a=>{const b=c(a).host;return k(b)});b.wbg.__wbg_href_706b235ecfe6848c=function(){return D(((b,d)=>{const e=c(d).href;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/a1+ W]=g;r()[b/a1+ U]=f}),arguments)};b.wbg.__wbg_pathname_5449afe3829f96a1=function(){return D(((b,d)=>{const e=c(d).pathname;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/a1+ W]=g;r()[b/a1+ U]=f}),arguments)};b.wbg.__wbg_search_489f12953342ec1f=function(){return D(((b,d)=>{const e=c(d).search;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/a1+ W]=g;r()[b/a1+ U]=f}),arguments)};b.wbg.__wbg_hash_553098e838e06c1d=function(){return D(((b,d)=>{const e=c(d).hash;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/a1+ W]=g;r()[b/a1+ U]=f}),arguments)};b.wbg.__wbg_href_7bfb3b2fdc0a6c3f=((b,d)=>{const e=c(d).href;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/a1+ W]=g;r()[b/a1+ U]=f});b.wbg.__wbg_pathname_c5fe403ef9525ec6=((b,d)=>{const e=c(d).pathname;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/a1+ W]=g;r()[b/a1+ U]=f});b.wbg.__wbg_search_c68f506c44be6d1e=((b,d)=>{const e=c(d).search;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/a1+ W]=g;r()[b/a1+ U]=f});b.wbg.__wbg_setsearch_fd62f4de409a2bb3=((a,b,d)=>{c(a).search=j(b,d)});b.wbg.__wbg_hash_cdea7a9b7e684a42=((b,d)=>{const e=c(d).hash;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/a1+ W]=g;r()[b/a1+ U]=f});b.wbg.__wbg_sethash_9bacb48849d0016e=((a,b,d)=>{c(a).hash=j(b,d)});b.wbg.__wbg_new_67853c351755d2cf=function(){return D(((a,b)=>{const c=new URL(j(a,b));return k(c)}),arguments)};b.wbg.__wbg_newwithbase_6aabbfb1b2e6a1cb=function(){return D(((a,b,c,d)=>{const e=new URL(j(a,b),j(c,d));return k(e)}),arguments)};b.wbg.__wbg_value_d7f5bfbd9302c14b=((b,d)=>{const e=c(d).value;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/a1+ W]=g;r()[b/a1+ U]=f});b.wbg.__wbg_setvalue_090972231f0a4f6f=((a,b,d)=>{c(a).value=j(b,d)});b.wbg.__wbg_get_bd8e338fbd5f5cc8=((a,b)=>{const d=c(a)[b>>>U];return k(d)});b.wbg.__wbg_length_cd7af8117672b8b8=(a=>{const b=c(a).length;return b});b.wbg.__wbg_newnoargs_e258087cd0daa0ea=((a,b)=>{const c=new Function(j(a,b));return k(c)});b.wbg.__wbg_call_27c0f87801dedf93=function(){return D(((a,b)=>{const d=c(a).call(c(b));return k(d)}),arguments)};b.wbg.__wbg_new_72fb9a18b5ae2624=(()=>{const a=new a2();return k(a)});b.wbg.__wbg_self_ce0dbfc45cf2f5be=function(){return D((()=>{const a=self.self;return k(a)}),arguments)};b.wbg.__wbg_window_c6fb939a7f436783=function(){return D((()=>{const a=window.window;return k(a)}),arguments)};b.wbg.__wbg_globalThis_d1e6af4856ba331b=function(){return D((()=>{const a=globalThis.globalThis;return k(a)}),arguments)};b.wbg.__wbg_global_207b558942527489=function(){return D((()=>{const a=global.global;return k(a)}),arguments)};b.wbg.__wbg_from_89e3fc3ba5e6fb48=(a=>{const b=N.from(c(a));return k(b)});b.wbg.__wbg_instanceof_ArrayBuffer_836825be07d4c9d2=(a=>{let b;try{b=c(a) instanceof ArrayBuffer}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_call_b3ca7c6051f9bec1=function(){return D(((a,b,d)=>{const e=c(a).call(c(b),c(d));return k(e)}),arguments)};b.wbg.__wbg_isSafeInteger_f7b04ef02296c4d2=(a=>{const b=Number.isSafeInteger(c(a));return b});b.wbg.__wbg_entries_95cc2c823b285a09=(a=>{const b=a2.entries(c(a));return k(b)});b.wbg.__wbg_is_010fdc0f4ab96916=((a,b)=>{const d=a2.is(c(a),c(b));return d});b.wbg.__wbg_resolve_b0083a7967828ec8=(a=>{const b=Promise.resolve(c(a));return k(b)});b.wbg.__wbg_then_0c86a60e8fcfe9f6=((a,b)=>{const d=c(a).then(c(b));return k(d)});b.wbg.__wbg_buffer_12d079cc21e14bdb=(a=>{const b=c(a).buffer;return k(b)});b.wbg.__wbg_newwithbyteoffsetandlength_aa4a17c33a06e5cb=((a,b,d)=>{const e=new V(c(a),b>>>U,d>>>U);return k(e)});b.wbg.__wbg_new_63b92bc8671ed464=(a=>{const b=new V(c(a));return k(b)});b.wbg.__wbg_set_a47bac70306a19a7=((a,b,d)=>{c(a).set(c(b),d>>>U)});b.wbg.__wbg_length_c20a40f15020d68a=(a=>{const b=c(a).length;return b});b.wbg.__wbg_instanceof_Uint8Array_2b3bbecd033d19f6=(a=>{let b;try{b=c(a) instanceof V}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_newwithlength_e9b4878cebadb3d3=(a=>{const b=new V(a>>>U);return k(b)});b.wbg.__wbg_subarray_a1f73cd4b5b42fe1=((a,b,d)=>{const e=c(a).subarray(b>>>U,d>>>U);return k(e)});b.wbg.__wbg_set_1f9b04f170055d33=function(){return D(((a,b,d)=>{const e=Reflect.set(c(a),c(b),c(d));return e}),arguments)};b.wbg.__wbindgen_debug_string=((b,d)=>{const e=u(c(d));const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/a1+ W]=g;r()[b/a1+ U]=f});b.wbg.__wbindgen_throw=((a,b)=>{throw new T(j(a,b))});b.wbg.__wbindgen_memory=(()=>{const b=a.memory;return k(b)});b.wbg.__wbindgen_closure_wrapper910=((a,b,c)=>{const d=w(a,b,438,z);return k(d)});b.wbg.__wbindgen_closure_wrapper1246=((a,b,c)=>{const d=A(a,b,593,B);return k(d)});b.wbg.__wbindgen_closure_wrapper1294=((a,b,c)=>{const d=A(a,b,615,C);return k(d)});return b});function D(b,c){try{return b.apply(this,c)}catch(b){a.__wbindgen_exn_store(k(b))}}let a;const b=new N(O).fill(P);b.push(P,Q,!0,!1);let d=b.length;const g=typeof TextDecoder!==R?new TextDecoder(S,{ignoreBOM:!0,fatal:!0}):{decode:()=>{throw T(`TextDecoder not available`)}};if(typeof TextDecoder!==R){g.decode()};let h=Q;let l=U;const m=typeof TextEncoder!==R?new TextEncoder(S):{encode:()=>{throw T(`TextEncoder not available`)}};const n=typeof m.encodeInto===X?((a,b)=>m.encodeInto(a,b)):((a,b)=>{const c=m.encode(a);b.set(c);return {read:a.length,written:c.length}});let q=Q;let s=Q;const v=typeof a0===R?{register:()=>{},unregister:()=>{}}:new a0(b=>{a.__wbindgen_export_2.get(b.dtor)(b.a,b.b)});let x=O;let E=Q;export default M;export{L as initSync}