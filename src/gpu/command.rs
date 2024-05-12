pub trait Command {
    fn push_buffers(&self, commands: &mut CommandList);
}

#[derive(Default)]
pub struct CommandList {
    commands: Vec<wgpu::CommandBuffer>,
}

impl CommandList {
    pub fn push(&mut self, command: wgpu::CommandBuffer) {
        self.commands.push(command);
    }

    pub fn extend(&mut self, commands: impl IntoIterator<Item = wgpu::CommandBuffer>) {
        self.commands.extend(commands);
    }

    pub fn submit(self, queue: &wgpu::Queue) {
        queue.submit(self.commands);
    }
}
