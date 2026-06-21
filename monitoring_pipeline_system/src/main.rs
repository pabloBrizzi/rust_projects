struct PipelineState {
    processed_records: u64,
    errors: u64,
    running: bool,
    }
enum Event {
    Start,
    ProcessRecords(u64),
    Error,
    Stop,
}
impl PipelineState {
    fn start(&mut self){
        self.running=true;
    }
    fn processed_records(&mut self, records: u64){
        self.processed_records+=records;
    }
    fn register_error(&mut self){
        self.errors+=1;
    }
    fn stop(&mut self){
        self.running = false;
    }
    fn process(&mut self, event: Event) { 
        // TODO: Create a match expression to process the different message // variants using the methods defined above. 
        match event { 
            Event::Start => self.start(), 
            Event::ProcessRecords(records) => self.processed_records(records), 
            Event::Error => self.register_error(), 
            Event::Stop => self.stop(), 
        } 
    } 

}
fn main() {
    
}

#[cfg(test)] mod tests { 
    use super::*; #[test] 
    fn test_match_message_call() { 
        let mut pipelinestate  = PipelineState { 
            processed_records: 0,
            errors: 0,
            running: false,
        }; 
        pipelinestate.process(Event::Start); 
        pipelinestate.process(Event::ProcessRecords(100));
        pipelinestate.process(Event::ProcessRecords(250));
        pipelinestate.process(Event::Error);
        pipelinestate.process(Event::Stop);
        
        assert_eq!(pipelinestate.processed_records, 350); 
        assert_eq!(pipelinestate.errors, 1); 
        assert_eq!(pipelinestate.running, false); 
    } 
}
