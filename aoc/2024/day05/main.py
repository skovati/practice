import pathlib
import networkx as nx

def parse_update(line: str):
    return [int(el) for el in line.split(",")]

def is_valid_order(G, update):
    for i in range(len(update)):
        for j in range(i+1, len(update)):
            if G.has_edge(update[j], update[i]):
                return False
    return True

def reorder_update(G, update):
    subgraph = G.subgraph(update)
    return list(nx.topological_sort(subgraph))

def build_graph(rules: str):
    G = nx.DiGraph()

    for rule in rules.splitlines():
        [first, second] = rule.split("|")
        G.add_edge(int(first), int(second))

    return G

def partion_updates(G, updates: str):
    updates = [parse_update(u) for u in updates.splitlines()]
    correct, incorrect = [], []
    for u in updates:
        if is_valid_order(G, u):
            correct.append(u)
        else:
            incorrect.append(u)
    return correct, incorrect

def solve_p1(path: str):
    contents = pathlib.Path(path).read_text()
    [rules, updates] = contents.split("\n\n")
    G = build_graph(rules)
    correct, _ = partion_updates(G, updates)

    return sum(u[len(u)//2] for u in correct)

def solve_p2(path: str):
    contents = pathlib.Path(path).read_text()

    [rules, updates] = contents.split("\n\n")
    G = build_graph(rules)
    correct, incorrect = partion_updates(G, updates)

    reorded = [reorder_update(G, u) for u in incorrect]

    return sum(u[len(u)//2] for u in reorded)

def main():
    for part, solve_func in [("p1", solve_p1), ("p2", solve_p2)]:
        for file in ["example.txt", "input.txt"]:
            print(f"{part} {file} solution: {solve_func(file)}")

if __name__ == "__main__":
    main()
