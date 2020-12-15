class Matrix:
    def __init__(self, matrix_string):
        self.contents = []
        for row in matrix_string.split("\n"):
            self.contents.append([int(num) for num in row.split(" ")])        

    def row(self, index):
        return self.contents[index - 1]

    def column(self, index):
        return [row[index - 1] for row in self.contents]
