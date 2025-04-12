from flask import Flask, jsonify
import json

app = Flask(__name__)

@app.route('/events', methods=['GET'])
def events():
    with open('sample_events.json', 'r') as f:
        data = json.load(f)
    return jsonify(data)

if __name__ == '__main__':
    app.run(debug=True, port=5000)
