# Technical requirements

* RQT-01 Use actix-web
* RQT-02 Use diesel
* RQT-03 Use MySQL
* RQT-04 Use DB cnnection pool R2D2
* RQT-05 Use Yew
* RQT-06 Use md-web components

# Dictionary

`Space` - An user-space or group-space with multiple wikis
`Wiki` - An some wiki

# Functional requirements

* [ ] RQF-01 Main wiki access URI is `witeka.org/<space>/<wiki>`
* [ ] RQF-02 Additional wiki access URI is `http://<space>.witeka.org/<wiki>`
* [ ] RQF-03 URI `witeka.org/<space>` by default redirect to `witeka.org/<space>/dashboard`
* [ ] RQF-04 URI `witeka.org/<space>` can be configured to redirect to main wiki: `witeka.org/<space>/<wiki>`


## Use cases

* [ ] RQF-LOGIN-01 User click on the login button
* [ ] RQF-LOGIN-02 User enter a name and a password
* [ ] RQF-LOGIN-03 User click on the login button
* [ ] RQF-LOGIN-04 User redirect to last viewed space or to your own space from witeka main page
