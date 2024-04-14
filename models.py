import numpy as np
import requests
import pandas as pd
from flask import Flask, request, jsonify
from sklearn.model_selection import train_test_split
from sklearn.preprocessing import MinMaxScaler
from tensorflow.keras.models import Sequential
from tensorflow.keras.layers import LSTM, Dense

app = Flask(__name__)

#empty variable, holds default model, can be changed by user
MODEL= None

class LSTMModel:
    def __init__(self, layers, neurones, data):

        #No shuffle, LSTM can take sequence of a batch into account
        self.X_train, self.X_test, self.Y_train, self.Y_test = train_test_split(data.drop(columns=["target"]), data["target"], test_size=0.2, shuffle=False, random_state=42)


        model = Sequential()
        for i in range(layers):
            if i == 0:
                #has 4 as input shape because thats number of features after PCA
                model.add(LSTM(neurones, input_shape=(self.X_train.shape[1], 1), return_sequences=True))
            elif i == layers - 1:
                # Last layer doesn't return sequences
                model.add(LSTM(neurones))
            else:
                model.add(LSTM(neurones, return_sequences=True))
        #Has on output as binary classification problem
        model.add(Dense(1, activation='sigmoid'))

        model.compile(loss='binary_crossentropy', optimizer='adam', metrics=['accuracy'])
        self.model = model

    def train(self, epochs, batch):
        #Binary loss because binary classification problem
        self.model.fit(self.X_train, self.Y_train, epochs=epochs, batch_size=batch)

    def test(self):
        loss, accuracy = self.model.evaluate(self.X_test, self.Y_test)
        return accuracy


def load_dataset():
    #No specific column names, made from PCA
    df = pd.read_csv('dataset/preprocess-test-network-attack.csv', header=None, names=['Column1', 'Column2', 'Column3', 'Column4'])

    #All packets malicious assumed
    df['target'] = 1

    return df


@app.route('/genmodel', methods=['POST'])
def genmodel():

    dataset = load_dataset()

    lstm_model = LSTMModel(int(request.form['layers']), int(request.form['neurons']), dataset)

    return "Data received successfully"


@app.route('/train', methods=['POST'])
def train():
    MODEL.train(int(request.form['epochs']), int(request.form['batch']))
    return "Model trained successfully"


@app.route('/test', methods=['GET'])
def test():
    accuracy = MODEL.test()
    print(accuracy)
    data = jsonify({ "accuracy" : accuracy})
    return data



if __name__ == "__main__":
    MODEL = LSTMModel(3, 35, load_dataset())
    MODEL.train(500,4)
    app.run(debug=True)
