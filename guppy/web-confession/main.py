from flask import Flask, request, redirect, render_template, send_from_directory, make_response
import openai
import sqlite3
import sqlparse
app = Flask(__name__)
openai.api_key = "REMOVED"


@app.route('/confess', methods=['POST'])
def receive_confession():
    confession = request.form.get('confession')
    answer = openai.Completion.create(
        engine="text-babbage-001",
        prompt="Is \"" + confession + "\" a confession? "
    )
  
    # Get the confession from the request
    if ("No" in answer['choices'][0]["text"] or "no" in answer['choices'][0]["text"]):
        return "Hey that's not a confession!"
    else:
        # insert confession into database
        # should be safe to use since chatgpt said so
        if "delete" in answer['choices'][0]["text"].lower() or "update" in answer['choices'][0]["text"].lower():
            return "OwO no delete or update pls "

        conn = sqlite3.connect("confessions.db")
        c = conn.cursor()

        c.execute("CREATE TABLE IF NOT EXISTS confessions (text TEXT)")
        # super secure sql inseration
        to_execute = sqlparse.split("INSERT INTO confessions (text) VALUES (\"" + confession + "\");");
        for i in to_execute:
            c.execute(i)
        comments = c.fetchall()
        conn.commit()
        conn.close()
        response = make_response("thanks for confessing" + str(comments), 200)
        response.mimetype = "text/plain"
        return response


def read_comments(db_file):
    # connect to the database
    conn = sqlite3.connect(db_file)
    c = conn.cursor()
    # execute a SELECT statement to retrieve book comments
    c.execute("SELECT * FROM confessions LIMIT 100")
    # fetch all the comments
    comments = c.fetchall()
    # close the connection
    conn.close()
    return comments

@app.route('/images/<path:filename>')
def serve_image(filename):
    return send_from_directory('imgs/',filename)

@app.route('/')
def index():
    comments = read_comments("confessions.db")
    return render_template('index.html',comments=comments)



if __name__ == '__main__':
    app.run(host="0.0.0.0")
