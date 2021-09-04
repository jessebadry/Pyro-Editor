const DocsList = document.getElementsByTagName("select")[0];
const DocumentAddButton = document.getElementById("add_but");


DocumentAddButton.addEventListener('click', (_) => {
    // Add new document

    let newDocElement = createDocListItem('');
    DocsList.appendChild(newDocElement);


});

function loadDocuments() {

    let documents = ffi.loadDocuments();
    documents.then((documents) => {
        console.log(JSON.stringify(documents));

    }).catch(ffi.onError);



}


// let saveButton = document.getElementById('save');

// saveButton.addEventListener('click', (_) => {

//     let docName = document.getElementById("title").value;
//     let text = document.getElementById("text").textContent;
//     console.log(docName, text);
//     window.ffi.saveDocument(docName, text);
// });
function createDocListItem(docName) {
    let textInput = document.createElement("input");

    textInput.type = "text";
    textInput.value = docName;
    let option = document.createElement("option");
    option.appendChild(textInput);

    return option;
}

function loadDocuments() {
    DocsList.innerHTML = null;
    let documents = ffi.loadDocuments();

    for (let i in documents) {
        let documentName = documents[i];
        let option = createDocListItem(documentName);
        DocsList.appendChild(option);
    }
}
loadDocuments();