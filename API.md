# To-Do List REST API

## Tasks

### Create a new task

* **Method:** `POST`
* **Path:** `/tasks`
* **Request body:** JSON object with task details.
  ```json
  {
    "title": "Buy milk",
    "due_date": "2024-03-10"
  }
  ```
* **Response:** JSON object with the created task, including its ID.
  ```json
  {
    "id": 1,
    "title": "Buy milk",
    "due_date": "2024-03-10",
    "completed": false,
    "dependencies": []
  }
  ```

### Get all tasks

* **Method:** `GET`
* **Path:** `/tasks`
* **Response:** JSON array of task objects.
  ```json
  [
    {
      "id": 1,
      "title": "Buy milk",
      "due_date": "2024-03-10",
      "completed": false,
      "dependencies": []
    },
    {
      "id": 2,
      "title": "Walk the dog",
      "due_date": null,
      "completed": true,
      "dependencies": [1]
    }
  ]
  ```

### Get a specific task

* **Method:** `GET`
* **Path:** `/tasks/{id}`
* **Response:** JSON object with the task details.
  ```json
  {
    "id": 1,
    "title": "Buy milk",
    "due_date": "2024-03-10",
    "completed": false,
    "dependencies": []
  }
  ```

### Update an existing task

* **Method:** `PUT`
* **Path:** `/tasks/{id}`
* **Request body:** JSON object with the updated task details.
  ```json
  {
    "title": "Buy almond milk",
    "completed": true
  }
  ```
* **Response:** JSON object with the updated task.
  ```json
  {
    "id": 1,
    "title": "Buy almond milk",
    "due_date": "2024-03-10",
    "completed": true,
    "dependencies": []
  }
  ```

### Delete a task

* **Method:** `DELETE`
* **Path:** `/tasks/{id}`
* **Response:** Empty response with 204 status code.

## Dependencies

### Add a dependency to a task

* **Method:** `POST`
* **Path:** `/tasks/{id}/deps`
* **Request body:** JSON object with the ID of the task to depend on.
  ```json
  {
    "dep_id": 5
  }
  ```
* **Response:** JSON object with the updated task.
  ```json
  {
    "id": 1,
    "title": "Buy milk",
    "due_date": "2024-03-10",
    "completed": false,
    "dependencies": [5]
  }
  ```

### Remove a dependency from a task

* **Method:** `DELETE`
* **Path:** `/tasks/{id}/deps/{dep_id}`
* **Response:** JSON object with the updated task.
  ```json
  {
    "id": 1,
    "title": "Buy milk",
    "due_date": "2024-03-10",
    "completed": false,
    "dependencies": []
  }
  ```
