var editor = new EditorJS('editor', {
    //...
    tools: {
        header: {
            class: Header,
            shortcut: 'CTRL+SHIFT+H',
            config: {
                placeholder: 'Enter a header',
                levels: [2, 3, 4],
                defaultLevel: 3
            }
        },

    }
    //...
});