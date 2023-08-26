window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', (event)=>{
    if (event.matches) {
        // ダークモードに切り替わった時の処理
        console.log('ダークモードに切り替わりました');
        isDarkMode = true
        window.document.body.classList.add("dark");
      } else {
        // ライトモードに切り替わった時の処理
        console.log('ライトモードに切り替わりました');
        isDarkMode = false
        window.document.body.classList.remove("dark");
      }
})
