build-puller :
	cd puller && go build -o ../pull puller.go

create-cookie-file :
	touch cookie.txt

init : build-puller create-cookie-file
