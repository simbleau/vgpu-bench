build: 
	c++ -fPIC -c -o renderer.o renderer.cpp && \
	c++ -shared -o renderer.so renderer.o && \
	rm renderer.o

clean:
	rm renderer.so