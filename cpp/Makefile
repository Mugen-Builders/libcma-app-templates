CXX  := g++

.PHONY: clean

app: app.cpp
	$(CXX) -O2 -std=c++20 -o $@ $< -lcma -lcmt

clean:
	@rm -rf app
	make -C clean
