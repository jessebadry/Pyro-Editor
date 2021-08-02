
const DocsList;

function loadDocuments(){

    let documents = ffi.loadDocuments();
    documents.then((documents) => {
        console.log(JSON.stringify(documents));

    }).catch(ffi.onError);
    
    
    
}


let saveButton = document.getElementById('save');

saveButton.addEventListener('click', (_) => {

    let docName = document.getElementById("title").value;
    let text = document.getElementById("text").textContent;
    console.log(docName, text);
    window.ffi.saveDocument(docName, text);
});