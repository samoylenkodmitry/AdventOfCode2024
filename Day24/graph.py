from graphviz import Digraph

def compute_logic_and_draw_graph(input_file):
    """
    Parse the input file, compute logic operations, and generate a Graphviz graph.

    :param input_file: Path to the input file containing the data in the specified format.
    :return: None
    """
    # Read input data from file
    with open(input_file, 'r') as file:
        input_data = file.read()

    # Initialize data storage
    inputs = {}
    operations = []

    # Parse the input data
    lines = input_data.strip().split('\n')
    for line in lines:
        if ":" in line:  # Input definition
            key, value = line.split(":")
            inputs[key.strip()] = int(value.strip())
        elif "->" in line:  # Operation definition
            operations.append(line.strip())

    # Initialize the graph
    graph = Digraph(format='png')
    graph.attr(rankdir='LR')

    # Add input nodes
    for key, value in inputs.items():
        graph.node(key, label=f"{key} = {value}", shape="ellipse")

    # Compute operations and add to the graph
    results = {}
    for op in operations:
        expression, result = op.split("->")
        result = result.strip()

        # Identify operation and split parts safely
        if " AND " in expression:
            parts = expression.split(" AND ")
            operation = "AND"
        elif " OR " in expression:
            parts = expression.split(" OR ")
            operation = "OR"
        elif " XOR " in expression:
            parts = expression.split(" XOR ")
            operation = "XOR"
        else:
            raise ValueError(f"Unsupported operation in expression: {expression}")

        # Extract and validate keys
        left, right = map(str.strip, parts)
        if left not in inputs or right not in inputs:
            raise KeyError(f"One or more keys not found in inputs: {left}, {right}")

        # Perform the operation
        if operation == "AND":
            results[result] = inputs[left] & inputs[right]
        elif operation == "OR":
            results[result] = inputs[left] | inputs[right]
        elif operation == "XOR":
            results[result] = inputs[left] ^ inputs[right]

        # Add the result to the graph
        graph.node(result, label=f"{result} = {results[result]}", shape="box")
        graph.edge(left, result)
        graph.edge(right, result)

    # Save the graph to a DOT file instead of rendering
    graph_file = "logic_graph.dot"
    graph.save(graph_file)
    print(f"Graph generated and saved as '{graph_file}'. Use Graphviz locally to render the graph.")

# Example usage
if __name__ == "__main__":
    input_file = "input.txt"
    compute_logic_and_draw_graph(input_file)

