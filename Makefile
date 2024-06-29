format:
	cd hello &&\
	make format &&\
	cd ..&&\
	cd calc&&\
	make format&&\
	cd ..

lint:
	cd hello&&\
	make lint&&\
	cd ..&&\
	cd calc&&\
	make lint&&\
	cd ..

test:
	cd hello&&\
	make test&&\
	cd ..&&\
	cd calc&&\
	make test&&\
	cd ..

run:
	cd hello&&\
	make run&&\
	cd ..&&\
	cd calc&&\
	make run&&\
	cd ..

git:
	git pull&&\
	git add -A &&\
	git commit&&\
	git push

all: format lint test run