function loadScript(scriptUrl) {
    const script = document.createElement('script');
    script.src = scriptUrl;
    document.body.appendChild(script);

    return new Promise((res, rej) => {
        script.onload = function () {
            res();
        }
        script.onerror = function () {
            rej();
        }
    });
}

// use
loadScript('/highlighter/highlight.min.js')
    .then(() => {
        hljs.highlightAll();
    })
    .catch(() => {
        console.error('Script loading failed! Handle this error');
    });