'use strict';



const ffi = {

    invoke: function(arg, commandName = 'invoke') {
        return window.__TAURI__.invoke(commandName, { arg: JSON.stringify(arg) })

        .catch(this.onError);

    },
    loadDocuments: () => window.__TAURI__.invoke('load_documents'),
    saveDocument: function(doc_name, text) {
        ffi.invoke({
            cmd: 'saveDocument',
            doc_name: doc_name,
            text: text
        });
    },
    loadMainPage: function() {
        window.location.replace("main.html");
    },
    onError: function(errorObj) {
        switch (errorObj.error_name) {
            case "NotEncryptedError":
                console.log("loading main page");
                ffi.loadMainPage();
                break;

            default:
                console.table(errorObj.error_name, errorObj.details);
        }
    },

    unlock: function(password) {
        ffi.invoke({
            cmd: 'crypt',
            password: password,
            locking: false,
        });
    },

    lock: function(password) {
        ffi.invoke({
            cmd: 'crypt',
            password: password,
            locking: true,
        });
    }
};

window.ffi = ffi;