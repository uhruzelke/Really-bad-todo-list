use std::ops::Not;

use iced::widget::{button, checkbox, column, row, text_input, Button, TextInput};
use iced::{Alignment, Element};





#[derive(Default,Clone)]
struct ToDoApp{
    new_todo: String,
    todos:Vec<ToDo>,
    currentl_edeting:Option<usize>,
    currentl_edeting_test:String
}

#[derive(Clone)]
struct ToDo{
    done: bool,
    description: String
}
#[derive(Clone,Debug)]
enum Message{
    ToggleToDo(usize),
    DeleatToDo(usize),
    AddToDo,
    InputChanged(String),
    InputEditingChanged(String),
    ClearAll,
    EditTodo(usize),
    DoneEditing


}


impl ToDoApp {
    fn update(&mut self,message:Message){
        match message {
            Message::ToggleToDo(t) => {
                                        if let Some(todo) = self.todos.get_mut(t){
                                            todo.done = ! todo.done;

                                        }
                                            },
            Message::DeleatToDo(t) => {
                                                self.todos.remove(t);
                                            },
            Message::AddToDo => {
                                        if !self.new_todo.is_empty(){
                                                    let description = self.new_todo.clone();
                                                    self.new_todo.clear();
                                                    self.todos.push(ToDo { done: false, description });
                                        };
                                             },
            Message::InputChanged(new_text) => {
                                        self.new_todo = new_text;
                                    },
            Message::ClearAll => {
                                self.todos.clear();
                            },
            Message::EditTodo(e) => {
                self.currentl_edeting_test.clear();
                        self.currentl_edeting = Some(e)
                    }
            Message::DoneEditing =>{
                        if let Some(todo) = self.todos.get_mut(self.currentl_edeting.unwrap()){
                            todo.description = self.currentl_edeting_test.clone();
                        }
                        self.currentl_edeting = None;
                    },
            Message::InputEditingChanged(t) => {
                self.currentl_edeting_test = t;
            },
        }
    }
    fn view(&self) -> Element<Message>{
        let add_todo_filed:TextInput<Message> = text_input("new todo here", &self.new_todo).on_input(Message::InputChanged).on_submit(Message::AddToDo);
        let add_todo_button = button("add todo").on_press(Message::AddToDo);
        let clar_all_button:Button<Message> = button("delat all todo").on_press(Message::ClearAll);
        let top_input = row![add_todo_filed, add_todo_button,clar_all_button].spacing(10);
        let todos = 
            self.todos
            .iter()
            .enumerate()
            .fold(column![].spacing(10), |column, (i, todo)| {
                column.push(
                row![
                    checkbox(&todo.description, todo.done).on_toggle(move |_| {
                            Message::ToggleToDo(i)
                        }),
                    button("Deleat").on_press(Message::DeleatToDo(i)),
                    button("Edit").on_press(Message::EditTodo(i))
                ].spacing(20)
                .align_y(Alignment::Center),
                )

            });
        if self.currentl_edeting.is_some(){
            let eddit_text_e:TextInput<Message> = text_input("new text here", &self.currentl_edeting_test).on_input(Message::InputEditingChanged).on_submit(Message::DoneEditing);
            let eddit_shit_butt = button("finis").on_press(Message::DoneEditing);
        let top_input = row![eddit_text_e,eddit_shit_butt].spacing(10);
        return column![top_input].padding(20).spacing(10).into()


        }
        column![top_input,todos].padding(20).spacing(10).into()
    }
    
}


fn main()-> iced::Result {
    iced::application("TODO", ToDoApp::update, ToDoApp::view)
        .theme(|_| iced::Theme::GruvboxDark)
        .centered()
        .run()
}
