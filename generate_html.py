import random
with open("./client_app_output/index.html", "w") as f:
    template = open("./client_app_output/index_template.html")
    index = ""
    version = str(random.randint(0, 1000000))
    for line in template.readlines():
        num = line.find(".js")
        if num != -1:
            line = line[:num+3] + "?v=" + version + line[num+3:]
        index += line
    f.write(index)

template = open("./client_app_output/static/client.js")
js = ""
version = str(random.randint(0, 1000000))
for line in template.readlines():
    num = line.find(".wasm")
    if num != -1:
        line = line[:num+5] + "?v=" + version + line[num+5:]
    js += line
with open("./client_app_output/static/client.js", "w") as f:
    f.write(js)