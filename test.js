var rubtle = new Rubtle();

rubtle.set(5);
assert(5, rubtle.get(), "Damn");
rubtle.inc();
assert(6, rubtle.get(), "Damn");
print(rubtle.get());