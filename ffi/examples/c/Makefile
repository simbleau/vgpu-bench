build: 
	gcc -fPIC -c -o renderer.o renderer.c && \
	gcc -shared -o renderer.so renderer.o && \
	rm renderer.o

clean:
	rm renderer.so