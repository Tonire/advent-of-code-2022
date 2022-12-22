#include <filesystem>
#include <fstream>
#include <iostream>
#include <string>

struct File
{
	std::size_t size;
	std::string name;
};

struct Directory
{
	std::size_t size;
	std::string name;
	Directory* parent;
	std::vector<File*> files;
	std::vector<Directory*> child_dirs;

	Directory* get_dir(const std::string& dir_name)
	{
		for (Directory* current : child_dirs)
		{
			if (current->name == dir_name)
			{
				return current;
			}
		}
		return nullptr;
	}

	std::size_t calculate_dir_size()
	{
		size = 0;
		for (File* current_file : files)
		{
			size += current_file->size;
		}
		for (Directory* current_directory : child_dirs)
		{
			size += current_directory->calculate_dir_size();
		}
		return size;
	}

	void sort_dir()
	{
		sort(child_dirs.begin(), child_dirs.end(), [](const Directory* first, const Directory* second){
			return first->size >= second->size;
		});
		for (Directory* current_dir : child_dirs)
		{
			current_dir->sort_dir();
		}
	}

	std::vector<Directory*> get_if_size_less_than(const std::size_t comp_size)
	{
		std::vector<Directory*> ret_vec;

		std::copy_if(child_dirs.begin(), child_dirs.end(), std::back_inserter(ret_vec), [comp_size](Directory* current) {
			return current->size <= comp_size;
		});
		std::vector<Directory*> child_greater;
		for (Directory* child : child_dirs)
		{
			
			std::vector<Directory*> childre_dirs = child->get_if_size_less_than(comp_size);
			child_greater.insert(child_greater.end(), childre_dirs.begin(), childre_dirs.end());
		}
		ret_vec.insert(ret_vec.end(), child_greater.begin(), child_greater.end());
		return ret_vec;
	}
};

struct Command
{
	std::string name;
	std::string args;
	std::vector<std::string> output;
};

std::string read_file(std::filesystem::path path)
{
	std::ifstream input_file_stream(path, std::ios::in | std::ios::binary);
	const auto file_size = std::filesystem::file_size(path);
	std::string result(file_size, '\0');
	input_file_stream.read(result.data(), file_size);
	return result;
}

std::vector<std::string> split_string(std::string input, const std::string& delim, const std::size_t offset = 0)
{
	std::vector<std::string> ret_vec;
	std::string token;
	std::size_t pos = offset;
	while ((pos = input.find(delim)) != std::string::npos)
	{
		token = input.substr(0, pos);
		ret_vec.push_back(token);
		input.erase(0, pos + delim.length());
	}
	ret_vec.push_back(input);
	return ret_vec;
}

Command extract_command(const std::vector<std::string>& input_file, const std::size_t offset)
{
	const std::string current_line = input_file[offset];
	const std::vector<std::string> split_command = split_string(current_line, " ");
	std::vector<std::string> command_output;
	if (split_command[1] == "ls")
	{
		for (size_t i = offset + 1; i < input_file.size(); i++)
		{
			const std::string output_line = input_file[i];
			const std::size_t token_pos = output_line.find("$");
			if (token_pos != std::string::npos)
			{
				break;
			}
			command_output.push_back(output_line);
		}
	}
	Command ret_command {split_command[1], split_command.size() > 2 ? split_command[2] : "", command_output };
	return ret_command;
}

void part1(Directory file_system)
{
	file_system.sort_dir();
	std::size_t filtered_dirs_total_size = 0;
	std::vector<Directory*> filtered_child_dirs = file_system.get_if_size_less_than(100000);
	for (Directory* current : filtered_child_dirs)
	{
		filtered_dirs_total_size += current->size;
	}
	std::cout << "---------PART 1---------\n";
	std::cout << "filtered_dirs_total_size: " << filtered_dirs_total_size << "\n";
}

int main(int argc, char** argv)
{
	std::string file_content = read_file("input.txt");
	const std::vector<std::string> split_input = split_string(file_content, "\r\n");
	std::vector<Command> commands;
	for (std::size_t i = 0; i < split_input.size(); i++)
	{
		const std::string current_line = split_input[i];
		const std::size_t token_command_location = current_line.find("$");
		if (token_command_location != std::string::npos)
		{
			Command found_command = extract_command(split_input, i);
			commands.push_back(found_command);
		}
	}
	Directory file_system { 0, "/" };
	Directory* current_dir = &file_system;
	for (const Command current_command : commands)
	{
		if (current_command.name == "cd")
		{
			if (current_command.args == "/")
			{
				current_dir = &file_system;
				continue;
			}
			if (current_command.args == "..")
			{
				current_dir = current_dir->parent;
				continue;
			}
			current_dir = current_dir->get_dir(current_command.args);
		}
		else if (current_command.name == "ls")
		{
			for (const std::string current_output : current_command.output)
			{
				const std::vector<std::string> split_output = split_string(current_output, " ");
				if (split_output[0] == "dir")
				{
					Directory* child_dir = new Directory();
					child_dir->size = 0;
					child_dir->name = split_output[1];
					child_dir->parent = current_dir;
					current_dir->child_dirs.push_back(child_dir);
				}
				else
				{
					std::stringstream sstream(split_output[0]);
					std::size_t file_size = 0;
					sstream >> file_size;

					File* child_file = new File();
					child_file->size = file_size;
					child_file->name = split_output[1];
					current_dir->files.push_back(child_file);
				}
			}
		}
	}
	file_system.calculate_dir_size();
	part1(file_system);
	return 0;
}