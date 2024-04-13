import numpy as np
import requests
import pandas as pd
from sklearn.model_selection import train_test_split
from sklearn.preprocessing import MinMaxScaler
from tensorflow.keras.models import Sequential
from tensorflow.keras.layers import LSTM, Dense

class LSTMModel:
    def __init__(layers, neurones, data)
        model = Sequential()
        for i in range(layers):
            if i == 0:
                # First layer needs input_shape parameter
                model.add(LSTM(hidden_sizes[i], input_shape=(None, input_dim), return_sequences=True))
            elif i == num_layers - 1:
                # Last layer doesn't return sequences
                model.add(LSTM(hidden_sizes[i]))
            else:
                model.add(LSTM(hidden_sizes[i], return_sequences=True))
        # Add output layer
        model.add(Dense(output_dim, activation='softmax'))
        self.model = model
        self.data = data

    def train():
        

def main():

    # Build LSTM model
    model = Sequential([
        LSTM(50, input_shape=(X_train.shape[1], X_train.shape[2]), return_sequences=True),
        LSTM(50),
        Dense(1)
    ])


if __name__ == "__main__":
    main()
