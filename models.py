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
    def __init__(self, layers, neurones, data, test_data):

        #No shuffle, LSTM can take sequence of a batch into account
        #Just used to format data to the correct shape
        self.X_train = data.drop(columns=["target"])
        self.Y_train = data["target"]

        self.X_test = test_data.drop(columns=["target"])
        self.Y_test = test_data["target"]

        shuffled_test_data = test_data.sample(frac=1).reset_index(drop=True)

        self.X_test_shuffle = shuffled_test_data.drop(columns=["target"])
        self.Y_test_shuffle = shuffled_test_data["target"]

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
        shuffle_loss, shuffle_accuracy = self.model.evaluate(self.X_test_shuffle, self.Y_test_shuffle)
        return [loss, accuracy, shuffle_loss, shuffle_accuracy]


def load_dataset(malicious_location, web_location):
    #No specific column names, made from PCA
    malicious_df = pd.read_csv(malicious_location, header=None, names=['Column1', 'Column2', 'Column3', 'Column4'])

    #All packets malicious assumed
    malicious_df['target'] = 1


    standard_df = pd.read_csv(web_location, header=None, names=['Column1', 'Column2', 'Column3', 'Column4'])

    standard_df['target'] = 0

    #Make sure datasets are 50% malicious/non malicious traffic
    if len(malicious_df) > len(standard_df):
        malicious_df = malicious_df[:len(standard_df)]
    else:
        standard_df = standard_df[:len(malicious_df)]

    df = pd.concat([malicious_df, standard_df])

    return df


@app.route('/genmodel', methods=['POST'])
def genmodel():

    MODEL = LSTMModel(int(request.form['layers']), int(request.form['neurons']),load_dataset('dataset/preprocessed/preprocess-dataset-attack.csv', 'dataset/preprocessed/preprocess-test-network-standard-webtraffic.csv'),load_dataset('dataset/preprocessed/preprocess-test-network-attack.csv', 'dataset/preprocessed/preprocess-test-network-standard-webtraffic-validate.csv'))

    return "Data received successfully"


@app.route('/train', methods=['POST'])
def train():
    MODEL.train(int(request.form['epochs']), int(request.form['batch']))
    return "Model trained successfully"


@app.route('/test', methods=['POST'])
def test():
    accuracy = MODEL.test()
    print("Loss: {}, Accuracy: {}, Shuffle Loss: {}, Shuffle Accuracy: {}".format(accuracy[0],accuracy[1],accuracy[2],accuracy[3]))
    data = jsonify({ "accuracy" : accuracy[1]})
    return data



if __name__ == "__main__":
    MODEL = LSTMModel(3, 35, load_dataset('dataset/preprocessed/preprocess-dataset-attack.csv', 'dataset/preprocessed/preprocess-test-network-standard-webtraffic.csv'),load_dataset('dataset/preprocessed/preprocess-test-network-attack.csv', 'dataset/preprocessed/preprocess-test-network-standard-webtraffic-validate.csv'))
    #MODEL.train(500,4)
    app.run(debug=True)
