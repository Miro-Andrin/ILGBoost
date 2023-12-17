import ILGBoost
import numpy as np

from ILGBoost import DataOrder


# class example:
#     def __init__(self) -> None:
#         pass
    
#     def fit(self, X, y):
#         pass

#     def predict(self, X):
#         pass

# model = example()
# model.fit(X, y)
# predictions = model.predict(X_new)

class Config:

    def __init__(self,steps: int = 100, gamma: float = 0.5, data_order: DataOrder = DataOrder.RowMajor, fit_intercept: bool = True):
        self.steps = steps
        self.gamma = gamma
        self.fit_intercept = fit_intercept

        if (type(data_order) == ILGBoost.DataOrder):
            self.data_order = data_order
        else:
            data_order = str(data_order).lower().strip()
            if (data_order == "row_major"):
                self.data_order = ILGBoost.DataOrder.RowMajor
            elif (data_order == "column_major" or data_order == "col_major"): 
                self.data_order = ILGBoost.DataOrder.ColumnMajor
            else:
                raise ValueError(f"data_order: {data_order} was not one of data_order or column_order")

        assert int(steps) == steps
        assert steps > 0
        assert float(gamma) == gamma
        assert 0 <= gamma and gamma <= 1

        




config = Config()

data = [1, 2, 3, 4, 5, 6]
y = [0,1,2]
result = ILGBoost.calculate(config, data, y)
print(result)


# True result
from numpy.linalg import inv
data = np.array(data).reshape((len(y),len(data)//len(y)))
y = np.array(y).reshape(len(y),1)

beta = inv(data.T@data)@data.T@y
print(f"Actual beta: {beta}")